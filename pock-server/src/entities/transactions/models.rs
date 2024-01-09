use rocket::serde::{Deserialize, Serialize};
use diesel::prelude::*;

use crate::trips::models::*;
use crate::users::models::*;

use pock_server::schema::trip::dsl::*;
use pock_server::schema::user::dsl::*;

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TransactionResponseDTO {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub value: f64,
    pub trip: TripResponseDTO,
    pub payer: UserResponseDTO,
    pub participants: Vec<UserResponseDTO>,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TransactionRequestDTO {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub value: Option<f64>,
    #[serde(rename = "tripId")]
    pub trip_id: Option<i64>,
    #[serde(rename = "payerId")]
    pub payer_id: Option<i64>,
    #[serde(rename = "participantsId")]
    pub participants_id: Vec<i64>,
}

#[derive(Identifiable, Queryable, Selectable, Serialize, Deserialize, Associations, Debug, Clone)]
#[diesel(table_name = pock_server::schema::transaction)]
#[diesel(belongs_to(trip, foreign_key = tripId))]
#[diesel(belongs_to(user, foreign_key = payerId))]
#[serde(crate = "rocket::serde")]
pub struct ReadTransactionEntity {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub value: f64,
    #[diesel(column_name = "tripId")]
    pub trip_id: i64,
    #[diesel(column_name = "payerId")]
    pub payer_id: i64,
}

#[derive(Insertable, Serialize, Deserialize, Associations, Debug, AsChangeset)]
#[diesel(table_name = pock_server::schema::transaction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(ReadTripEntity, foreign_key = tripId))]
#[diesel(belongs_to(ReadUserEntity, foreign_key = payerId))]
#[serde(crate = "rocket::serde")]
pub struct UpdateTransactionEntity {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub value: Option<f64>,
    #[diesel(column_name = "tripId")]
    pub trip_id: Option<i64>,
    #[diesel(column_name = "payerId")]
    pub payer_id: Option<i64>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Associations, Identifiable, Debug)]
#[diesel(table_name = pock_server::schema::transaction_participants_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(ReadTransactionEntity, foreign_key = transactionId))]
#[diesel(belongs_to(ReadUserEntity, foreign_key = userId))]
#[diesel(primary_key(transactionId, userId))]
#[serde(crate = "rocket::serde")]
pub struct ReadTransactionParticipantEntity {
    #[diesel(column_name = "transactionId")]
    pub transaction_id: i64,
    #[diesel(column_name = "userId")]
    pub user_id: i64,
}

#[derive(Insertable, Serialize, Deserialize, Associations, Identifiable, Debug)]
#[diesel(table_name = pock_server::schema::transaction_participants_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(ReadTransactionEntity, foreign_key = transactionId))]
#[diesel(belongs_to(ReadUserEntity, foreign_key = userId))]
#[diesel(primary_key(transactionId, userId))]
#[serde(crate = "rocket::serde")]
pub struct UpdateTransactionParticipantEntity {
    #[diesel(column_name = "transactionId")]
    pub transaction_id: Option<i64>,
    #[diesel(column_name = "userId")]
    pub user_id: Option<i64>,
}