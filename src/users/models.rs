use diesel::QueryDsl;

use crate::schema;

#[derive(Debug, Queryable)]
pub struct User {
    pub user_id: u32,
    pub email: String,
    //password: String,
    pub nickname: String,
    //birthday: Date,
    pub access: u32,
    pub created: i32,
    pub lastlogin: i32,
    pub options: u32,
    pub messages: i32,
    pub avatar: String,
    pub city_name: String,
    pub region_name: String,
    pub country_name: String,
    pub FIO: String,
    pub job: String,
    pub is_activated: i8,
}

type AllColumns = (
    schema::users::columns::user_id,
    schema::users::columns::email,
    //schema::users::columns::password,
    schema::users::columns::nickname,
    //schema::users::columns::birthday,
    schema::users::columns::access,
    schema::users::columns::created,
    schema::users::columns::lastlogin,
    schema::users::columns::options,
    schema::users::columns::messages,
    schema::users::columns::avatar,
    schema::users::columns::city_name,
    schema::users::columns::region_name,
    schema::users::columns::country_name,
    schema::users::columns::FIO,
    schema::users::columns::job,
    schema::users::columns::is_activated,
);

const ALL_COLUMNS: AllColumns = (
    schema::users::columns::user_id,
    schema::users::columns::email,
    //schema::users::columns::password,
    schema::users::columns::nickname,
    //schema::users::columns::birthday,
    schema::users::columns::access,
    schema::users::columns::created,
    schema::users::columns::lastlogin,
    schema::users::columns::options,
    schema::users::columns::messages,
    schema::users::columns::avatar,
    schema::users::columns::city_name,
    schema::users::columns::region_name,
    schema::users::columns::country_name,
    schema::users::columns::FIO,
    schema::users::columns::job,
    schema::users::columns::is_activated,
);

impl User {
    pub fn select() -> diesel::dsl::Select<schema::users::table, AllColumns> {
        schema::users::dsl::users.select(ALL_COLUMNS)
    }

    pub fn find(
        user_id: u32,
    ) -> diesel::dsl::Find<diesel::dsl::Select<schema::users::table, AllColumns>, u32> {
        Self::select().find(user_id)
    }

    pub fn is_admin(&self) -> bool {
        (self.access & 0x8000) != 0
    }
}
