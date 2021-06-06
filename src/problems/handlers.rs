use diesel::r2d2::{ConnectionManager, Pool};
use wamp_async::{WampArgs, WampKwArgs};

pub fn get_problem_details<'a>(
    pool: std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>,
) -> wamp_async::RpcFunc<'a> {
    Box::new(
        move |_args: Option<WampArgs>, kwargs: Option<WampKwArgs>| -> wamp_async::RpcFuture {
            let pool = std::sync::Arc::clone(&pool);
            Box::pin(async move {
                let input: super::resolvers::GetProblemDetailsRequest =
                    crate::helpers::try_from_kwargs_required(kwargs)?;

                let resolved_input =
                    super::resolvers::ProblemDetails::resolve(input, &pool).await?;

                let problem =
                    super::resolvers::GetProblemDetailsResponse::from(resolved_input.problem);
                let value = wamp_async::try_into_kwargs(problem)?;
                Ok((None, Some(value)))
            })
        },
    )
}
