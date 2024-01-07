use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserResponseDTO {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserRequestDTO {
    pub id: Option<i64>,
    pub name: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = pock_server::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct ReadUserEntity {
    pub id: i64,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = pock_server::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct UpdateUserEntity {
    pub id: Option<i64>,
    pub name: String,
}