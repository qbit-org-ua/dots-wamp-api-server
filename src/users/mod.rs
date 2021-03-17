mod handlers;
pub mod models;
pub mod resolvers;

pub async fn register(
    client: &wamp_async::Client<'_>,
    endpoint_prefix: &str,
    pool: &crate::DbPool,
) -> Result<Vec<wamp_async::WampId>, wamp_async::WampError> {
    let get_user_details_rpc_id = client
        .register(
            &format!("{}.users.get_user_details", endpoint_prefix),
            self::handlers::get_user_details(pool.clone()),
        )
        .await?;

    Ok(vec![get_user_details_rpc_id])
}
