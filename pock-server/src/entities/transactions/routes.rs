#[allow(unused_imports)]
use diesel::prelude::*;
mod models;
#[allow(unused_imports)]
use crate::transactions::models::*;
#[allow(unused_imports)]
use crate::users::models::*;
use crate::trips::models::*;


use pock_server::establish_connection;
#[allow(unused_imports)]
use pock_server::schema::transaction::dsl::*;
#[allow(unused_imports)]
use pock_server::schema::user::dsl::*;
use pock_server::schema::trip::dsl::*;
#[allow(unused_imports)]
use pock_server::schema::transaction_participants_user::dsl::*;
#[allow(unused_imports)]
use rocket::serde::json::{Json, Value, json};

#[get("/")]
pub fn list() -> Value {
    let connection = &mut establish_connection();

    let read_transactions = transaction
        .inner_join(trip)
        .inner_join(user)
        .load::<(ReadTransactionEntity, ReadTripEntity, ReadUserEntity)>(connection)
        .expect("Error loading transactions");

    let transactions_dto: Vec<TransactionResponseDTO> = read_transactions
        .into_iter()
        .map(|(ctra,ctrip,cpayer)| TransactionResponseDTO {
            id: ctra.id,
            name: ctra.name.clone(),
            value: ctra.value.clone(),
            description: ctra.description.clone(),
            payer: UserResponseDTO {
                id: cpayer.id,
                name: cpayer.name
            },
            trip: TripResponseDTO {
                id: ctrip.id,
                name: ctrip.name,
                description: ctrip.description
            },
            participants: get_transaction_participants(&ctra).map(|(cpart, upart)| UserResponseDTO {
                id: upart.id,
                name: upart.name
            }).collect()

        }).collect();
    json!(transactions_dto)
}

fn get_transaction_participants(rtrans: &ReadTransactionEntity) -> std::vec::IntoIter<(ReadTransactionParticipantEntity, ReadUserEntity)>{
    let connection = &mut establish_connection();

    ReadTransactionParticipantEntity::belonging_to(rtrans)
        .inner_join(user)
        .load(connection)
        .expect("Error loading participants")
        .into_iter()
}