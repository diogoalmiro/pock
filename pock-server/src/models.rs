use diesel::prelude::*;
use rocket::serde::json::Value;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub name: String,
}
/*
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(User), belongs_to(Trip))]
#[diesel(table_name = crate::schema::transaction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Transaction {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub value: f64,
    pub tripId: Option<i64>,
    pub payerId: Option<i64>
}

#[derive(Insertable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(belongs_to(User), belongs_to(Trip))]
#[diesel(table_name = crate::schema::transaction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct NewTransaction {
    pub name: String,
    pub description: String,
    pub value: f64,
    pub tripId: Option<i64>,
    pub payerId: Option<i64>,
    pub participants: Vec<i64>
}
*/

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TransactionResponseDTO {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub value: f64,
    pub trip: Value,
    pub payer: User,
    pub participants: Vec<User>,
}

#[derive(Selectable, Queryable, Serialize, Debug)]
#[diesel(table_name = crate::schema::transaction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Trip), belongs_to(User))]
#[serde(crate = "rocket::serde")]
pub struct Transaction {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub value: f64,
    #[diesel(column_name = "tripId")]
    pub trip_id: i64,
    #[diesel(column_name = "payerId")]
    pub payer_id: i64,
}

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = crate::schema::transaction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Trip), belongs_to(User))]
#[serde(crate = "rocket::serde")]
pub struct NewTransaction {
    pub name: String,
    pub description: String,
    pub value: f64,
    #[diesel(column_name = "tripId")]
    pub trip_id: i64,
    #[diesel(column_name = "payerId")]
    pub payer_id: i64,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TransactionRequestDTO {
    pub name: String,
    pub description: String,
    pub value: f64,
    #[serde(rename = "tripId")]
    pub trip_id: i64,
    #[serde(rename = "payerId")]
    pub payer_id: i64,
    pub participants: Vec<i64>,
}
