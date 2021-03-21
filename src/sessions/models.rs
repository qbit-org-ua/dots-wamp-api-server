use std::convert::TryFrom;

use diesel::QueryDsl;

use crate::schema;

#[derive(Debug, Queryable)]
pub struct Session {
    session_id: String,
    user_agent: String,
    created_ip: u32,
    updated_ip: u32,
    created: i32,
    lifetime: i32,
    expire: i32,
    session_data: String,
}

type AllColumns = (
    schema::sessions::columns::session_id,
    schema::sessions::columns::user_agent,
    schema::sessions::columns::created_ip,
    schema::sessions::columns::updated_ip,
    schema::sessions::columns::created,
    schema::sessions::columns::lifetime,
    schema::sessions::columns::expire,
    schema::sessions::columns::session_data,
);

const ALL_COLUMNS: AllColumns = (
    schema::sessions::columns::session_id,
    schema::sessions::columns::user_agent,
    schema::sessions::columns::created_ip,
    schema::sessions::columns::updated_ip,
    schema::sessions::columns::created,
    schema::sessions::columns::lifetime,
    schema::sessions::columns::expire,
    schema::sessions::columns::session_data,
);

impl Session {
    pub fn select() -> diesel::dsl::Select<schema::sessions::table, AllColumns> {
        schema::sessions::dsl::sessions.select(ALL_COLUMNS)
    }

    pub fn find(
        session_id: String,
    ) -> diesel::dsl::Filter<
        diesel::dsl::Select<schema::sessions::table, AllColumns>,
        diesel::dsl::And<
            diesel::dsl::Gt<schema::sessions::dsl::expire, i32>,
            diesel::dsl::Eq<schema::sessions::dsl::session_id, String>,
        >,
    > {
        use crate::diesel::ExpressionMethods;
        Self::select()
            .filter(
                schema::sessions::dsl::expire.gt(i32::try_from(chrono::Local::now().timestamp())
                    .expect("it is time to migrate to i64 session expiration")),
            )
            .find(session_id)
    }

    pub fn user_id(&self) -> Option<u32> {
        const UID_FIELD_IDENTIFIER: &str = "uid|i:";
        let uid_field_position = self.session_data.find(UID_FIELD_IDENTIFIER)?;
        let trimmed_left_session_data =
            &self.session_data[(uid_field_position + UID_FIELD_IDENTIFIER.len())..];
        let uid_str = &trimmed_left_session_data[..trimmed_left_session_data.find(';')?];
        uid_str.parse().ok()
    }
}
