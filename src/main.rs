use clap::Clap;
use diesel::r2d2::{ConnectionManager, Pool};

#[macro_use]
extern crate diesel;

use wamp_async::{Client, ClientConfig, ClientState, SerializerType};

mod contests;
mod helpers;
//mod problems;
mod schema;
mod sessions;
mod solutions;
mod users;

pub type DbPool = std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>;

#[derive(Debug, Clap)]
#[clap(
    version,
    author,
    about,
    setting(clap::AppSettings::ColoredHelp),
    setting(clap::AppSettings::DisableHelpSubcommand),
    setting(clap::AppSettings::VersionlessSubcommands),
    setting(clap::AppSettings::NextLineHelp)
)]
pub struct Config {
    #[clap(long, default_value = "mysql://dots:dots_password@127.0.0.1:3306/dots")]
    dots_db_url: url::Url,
    #[clap(long, default_value = "ws://localhost:8090/ws")]
    dots_wamp_url: url::Url,
    #[clap(long, default_value = "dots.2020")]
    dots_wamp_common_prefix: String,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config = Config::parse();
    //use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
    //let dt = DateTime::<Local>::from_utc(NaiveDateTime::from_timestamp(1602572652, 0), Utc);
    //let dt = Local.timestamp(1602572652, 0);
    //let dt = Local::now();
    //println!("dt: {:?}", dt.timestamp());

    let manager = ConnectionManager::<diesel::MysqlConnection>::new(config.dots_db_url.as_str());
    let pool = std::sync::Arc::new(Pool::builder().build(manager)?);

    // Connect to the server
    let (mut client, (event_loop, rpc_event_queue)) = Client::connect(
        &config.dots_wamp_url,
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
    endpoint_rpc_ids
        .extend(crate::users::register(&client, &config.dots_wamp_common_prefix, &pool).await?);
    endpoint_rpc_ids
        .extend(crate::solutions::register(&client, &config.dots_wamp_common_prefix, &pool).await?);
    endpoint_rpc_ids
        .extend(crate::contests::register(&client, &config.dots_wamp_common_prefix, &pool).await?);

    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use tokio_diesel::{AsyncRunQueryDsl, OptionalExtension};

    use crate::schema;

    let latest_solution: Option<crate::solutions::models::Solution> =
        crate::solutions::models::Solution::select()
            .order(schema::solutions::dsl::posted_time.desc())
            .first_async(&pool)
            .await
            .optional()?;
    let mut latest_checked_time: u32 = latest_solution
        .map(|solution| solution.checked_time)
        .unwrap_or(0);

    loop {
        if !client.is_connected() {
            break;
        }

        let new_solutions: Vec<crate::solutions::models::Solution> =
            crate::solutions::models::Solution::select()
                .filter(schema::solutions::dsl::checked_time.gt(latest_checked_time))
                .order(schema::solutions::dsl::checked_time)
                .limit(10)
                .load_async(&pool)
                .await?;

        if let Some(latest_solution) = new_solutions.last() {
            latest_checked_time = latest_solution.checked_time;
        } else {
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            continue;
        }

        let value = wamp_async::try_into_kwargs(
            crate::solutions::resolvers::RecentSolutionEvents::from(new_solutions),
        )?;
        println!("publishing... {:?}", value);

        client
            .publish(
                &format!("{}.solutions.recent", config.dots_wamp_common_prefix),
                None,
                Some(value),
                false,
            )
            .await
            .unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    if let ClientState::Disconnected(Err(e)) = client.get_cur_status() {
        println!("Client disconnected because of: {:?}", e);
        return Err(color_eyre::Report::msg("Unexpected disconnect"));
    }

    for endpoint_rpc_id in endpoint_rpc_ids {
        client.unregister(endpoint_rpc_id).await?;
    }

    println!("Leaving realm");
    client.leave_realm().await?;

    client.disconnect().await;

    Ok(())
}
