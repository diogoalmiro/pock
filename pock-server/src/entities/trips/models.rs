use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TripResponseDTO {
    pub id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TripRequestDTO {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = pock_server::schema::trip)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct ReadTripEntity {
    pub id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = pock_server::schema::trip)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct UpdateTripEntity {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}
