use diesel::r2d2::{ConnectionManager, Pool};
use wamp_async::{WampArgs, WampKwArgs};

pub fn get_solution_details<'a>(
    pool: std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>,
) -> wamp_async::RpcFunc<'a> {
    Box::new(
        move |_args: Option<WampArgs>, kwargs: Option<WampKwArgs>| -> wamp_async::RpcFuture {
            let pool = std::sync::Arc::clone(&pool);
            Box::pin(async move {
                let input: super::resolvers::GetSolutionDetailsRequest =
                    crate::helpers::try_from_kwargs_required(kwargs)?;

                let resolved_input =
                    super::resolvers::SolutionDetails::resolve(input, &pool).await?;

                let solution =
                    super::resolvers::GetSolutionDetailsResponse::from(resolved_input.solution);
                let value = wamp_async::try_into_kwargs(solution)?;
                Ok((None, Some(value)))
            })
        },
    )
}
