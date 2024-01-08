#[allow(unused_imports)]
use diesel::prelude::*;
mod models;
#[allow(unused_imports)]
use crate::transactions::models::*;
#[allow(unused_imports)]
use crate::users::models::*;
use crate::trips::models::*;


use pock_server::establish_connection;
use pock_server::schema::transaction::dsl::*;
use pock_server::schema::user::dsl::user;
use pock_server::schema::trip::dsl::trip;

#[allow(unused_imports)]
use rocket::serde::json::{Json, Value, json};

#[get("/")]
pub fn list() -> Json<Vec<TransactionResponseDTO>> {
    let connection = &mut establish_connection();

    let read_full_transactions = transaction
        .inner_join(trip)
        .inner_join(user)
        .load::<(ReadTransactionEntity, ReadTripEntity, ReadUserEntity)>(connection)
        .expect("Error loading transactions");
    // extract transactions
    let extract_transactions: Vec<ReadTransactionEntity> =  read_full_transactions
        .iter()
        .map(|(t, _, _)| t.clone())
        .collect();
    let participants_transactions = ReadTransactionParticipantEntity::belonging_to(&extract_transactions)
        .inner_join(user)
        .load::<(ReadTransactionParticipantEntity, ReadUserEntity)>(connection)
        .expect("Error loading participants")
        .grouped_by(&extract_transactions);
    let transactions_dto: Vec<TransactionResponseDTO> = read_full_transactions
        .into_iter()
        .zip(participants_transactions)
        .map(|((ctra,ctrip,cpayer), pars)| TransactionResponseDTO {
            id: ctra.id,
            name: ctra.name.clone(),
            value: ctra.value,
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
            participants: pars.into_iter().map(|(_, u)| UserResponseDTO {
                id: u.id,
                name: u.name
            }).collect()
        }).collect();
    Json(transactions_dto)
}

#[get("/<param_id>")]
pub fn read(param_id: i64) -> Json<TransactionResponseDTO> {
    let connection = &mut establish_connection();

    let read_full_transaction = transaction
        .inner_join(trip)
        .inner_join(user)
        .filter(id.eq(param_id))
        .first::<(ReadTransactionEntity, ReadTripEntity, ReadUserEntity)>(connection)
        .expect("Error loading transaction");
    let participants_transaction = ReadTransactionParticipantEntity::belonging_to(&read_full_transaction.0)
        .inner_join(user)
        .load::<(ReadTransactionParticipantEntity, ReadUserEntity)>(connection)
        .expect("Error loading participants");
    let transaction_dto = TransactionResponseDTO {
        id: read_full_transaction.0.id,
        name: read_full_transaction.0.name.clone(),
        value: read_full_transaction.0.value,
        description: read_full_transaction.0.description.clone(),
        payer: UserResponseDTO {
            id: read_full_transaction.2.id,
            name: read_full_transaction.2.name
        },
        trip: TripResponseDTO {
            id: read_full_transaction.1.id,
            name: read_full_transaction.1.name,
            description: read_full_transaction.1.description
        },
        participants: participants_transaction.into_iter().map(|(_, u)| UserResponseDTO {
            id: u.id,
            name: u.name
        }).collect()
    };
    Json(transaction_dto)
}