use diesel::r2d2::{ConnectionManager, Pool};
use wamp_async::{WampArgs, WampKwArgs};

pub fn get_solution_details<'a>(
    pool: std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>,
) -> wamp_async::RpcFunc<'a> {
    Box::new(
        move |args: Option<WampArgs>, kwargs: Option<WampKwArgs>| -> wamp_async::RpcFuture {
            let pool = std::sync::Arc::clone(&pool);
            Box::pin(async move {
                //
                // let num_users: Vec<crate::users::models::User> =
                //     crate::users::models::User::select()
                //         .limit(3)
                //         .get_results_async(&pool)
                //         .await
                //         .unwrap();
                // println!("echo_db: {:?}", num_users);

                // println!("peer.echo {:?} {:?}", args, kwargs);
                // let q: Q = wamp_async::try_from_kwargs(kwargs.unwrap())?;

                // println!("Q: {:?}", q);

                // //wamp_async::try_into_kwargs([("asd", 1)]).unwrap();
                // let value = wamp_async::try_into_kwargs(q)?;
                let input: super::resolvers::SolutionDetailsRequest =
                    wamp_async::try_from_kwargs(kwargs.ok_or_else(|| {
                        wamp_async::WampError::UnknownError("kwargs are required".to_string())
                    })?)?;
                let resolved_input =
                    super::resolvers::SolutionDetailsRequestResolver::resolve(input, &pool)
                        .await
                        .map_err(|err| {
                            wamp_async::WampError::UnknownError("resolve err".to_string())
                        })?;

                let solution: super::resolvers::SolutionDetailsResponse =
                    resolved_input.solution.into();
                let value = wamp_async::try_into_kwargs(solution)?;
                Ok((args, Some(value)))
            })
        },
    )
}
