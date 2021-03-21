pub(crate) type DbPool =
    std::sync::Arc<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>>;

pub(crate) fn try_from_kwargs_required<'a, T: serde::de::DeserializeOwned>(
    value: Option<wamp_async::WampKwArgs>,
) -> Result<T, wamp_async::WampError> {
    if let Some(value) = value {
        wamp_async::try_from_kwargs(value)
    } else {
        Err(wamp_async::WampError::UnknownError(
            "kwargs are required".to_string(),
        ))
    }
}
