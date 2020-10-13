mod handlers;
pub mod models;
pub mod resolvers;

pub async fn register(
    client: &wamp_async::Client,
    endpoint_prefix: &str,
    pool: &crate::DbPool,
) -> Result<Vec<wamp_async::WampId>, wamp_async::WampError> {
    let get_solution_details_rpc_id = client
        .register(
            &format!("{}.solutions.get_solution_details", endpoint_prefix),
            self::handlers::get_solution_details(pool.clone()),
        )
        .await?;

    Ok(vec![get_solution_details_rpc_id])
}
