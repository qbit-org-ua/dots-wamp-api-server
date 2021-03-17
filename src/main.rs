use std::error::Error;

use diesel::r2d2::{ConnectionManager, Pool};

#[macro_use]
extern crate diesel;

use wamp_async::{Client, ClientConfig, ClientState, SerializerType};

mod schema;
mod sessions;
mod solutions;
mod users;

pub type DbPool = std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>;

const WAMP_COMMON_PREFIX: &'static str = "dots.2020";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

    //let dt = DateTime::<Local>::from_utc(NaiveDateTime::from_timestamp(1602572652, 0), Utc);
    //let dt = Local.timestamp(1602572652, 0);
    let dt = Local::now();
    println!("dt: {:?}", dt.timestamp());

    let manager = ConnectionManager::<diesel::MysqlConnection>::new(
        "mysql://dots:dots_password@127.0.0.1:3306/dots",
    );
    let pool = std::sync::Arc::new(Pool::builder().build(manager)?);

    // Connect to the server
    let (mut client, (event_loop, rpc_event_queue)) = Client::connect(
        "ws://localhost:8090/ws".to_string().as_str(),
        Some(ClientConfig::default().set_serializers(vec![SerializerType::Json])),
    )
    .await?;
    println!("Connected");

    tokio::spawn(event_loop);

    // Handle RPC events in separate tasks
    tokio::spawn(async move {
        let mut rpc_event_queue = rpc_event_queue.unwrap();
        while let Some(rpc_event) = rpc_event_queue.recv().await {
            // Execute the function call
            tokio::spawn(rpc_event);
        }
    });

    println!("Joining realm");
    client
        .join_realm_with_authentication(
            "dots",
            vec![wamp_async::AuthenticationMethod::Ticket],
            "dots-backend",
            |_authentication_method, _extra| async {
                Ok(wamp_async::AuthenticationChallengeResponse::with_signature(
                    "back".into(),
                ))
            },
        )
        .await?;

    let mut endpoint_rpc_ids = Vec::new();
    endpoint_rpc_ids.extend(crate::users::register(&client, WAMP_COMMON_PREFIX, &pool).await?);
    endpoint_rpc_ids.extend(crate::solutions::register(&client, WAMP_COMMON_PREFIX, &pool).await?);

    loop {
        if !client.is_connected() {
            break;
        }
        println!("publishing...");
        client
            .publish(
                &format!("{}.solutions.recent", WAMP_COMMON_PREFIX),
                Some(vec![1.into()]),
                None,
                true,
            )
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    if let ClientState::Disconnected(Err(e)) = client.get_cur_status() {
        println!("Client disconnected because of: {:?}", e);
        return Err(From::from("Unexpected disconnect".to_string()));
    }

    for endpoint_rpc_id in endpoint_rpc_ids {
        client.unregister(endpoint_rpc_id).await?;
    }

    println!("Leaving realm");
    client.leave_realm().await?;

    client.disconnect().await;

    Ok(())
}
