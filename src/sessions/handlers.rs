use diesel::r2d2::{ConnectionManager, Pool};
use wamp_async::{WampArgs, WampKwArgs};

pub fn _sign_in<'a>(
    pool: std::sync::Arc<Pool<ConnectionManager<diesel::MysqlConnection>>>,
) -> wamp_async::RpcFunc<'a> {
    Box::new(
        move |_args: Option<WampArgs>, _kwargs: Option<WampKwArgs>| -> wamp_async::RpcFuture {
            let _pool = std::sync::Arc::clone(&pool);
            Box::pin(async move {
                //
                Ok((None, None))
            })
        },
    )
}
