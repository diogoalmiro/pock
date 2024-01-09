use diesel::result::Error;
use diesel::prelude::*;
mod models;
#[allow(unused_imports)]
use crate::transactions::models::*;
#[allow(unused_imports)]
use crate::users::models::*;
use crate::trips::models::*;


use pock_server::establish_connection;
use pock_server::schema::transaction::dsl::*;
use pock_server::schema::user::dsl::{user, id as user_id};
use pock_server::schema::trip::dsl::{trip, id as trip_id};
use pock_server::schema::transaction_participants_user::dsl::{transaction_participants_user, transactionId as transaction_participants_user_transaction_id};

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

#[post("/", data = "<data_transaction>")]
pub fn create(data_transaction: Json<TransactionRequestDTO>) -> Json<TransactionResponseDTO> {
    let connection = &mut establish_connection();

    let data_name = data_transaction.name.clone();
    let data_description = data_transaction.description.clone().or(data_name.clone());
    let data_value = data_transaction.value.expect("Value is required");
    let data_payer_id = data_transaction.payer_id.expect("Payer is required");
    let data_trip_id = data_transaction.trip_id.expect("Trip is required");
    let data_participants_id = data_transaction.participants_id.clone().expect("Participants is required");

    let inserted_transaction: std::result::Result<ReadTransactionEntity, Error> = connection.transaction(|connection| {
        let new_transaction = UpdateTransactionEntity {
            id: None,
            name: data_name,
            description: data_description,
            value: Some(data_value),
            trip_id: Some(data_trip_id),
            payer_id: Some(data_payer_id),
        };
        let inserted_transaction = diesel::insert_into(transaction)
            .values(&new_transaction)
            .get_result::<ReadTransactionEntity>(connection)
            .expect("Error saving new transaction");
        let participants = data_participants_id.clone().into_iter().map(|pid| UpdateTransactionParticipantEntity {
            transaction_id: Some(inserted_transaction.id),
            user_id: Some(pid)
        }).collect::<Vec<UpdateTransactionParticipantEntity>>();
        diesel::insert_into(transaction_participants_user)
            .values(&participants)
            .execute(connection)
            .expect("Error saving new transaction participants");
        Ok(inserted_transaction)
    });
    let inserted_transaction = inserted_transaction.expect("Error saving new transaction");
    let inserted_participants_transaction = ReadTransactionParticipantEntity::belonging_to(&inserted_transaction)
        .inner_join(user)
        .load::<(ReadTransactionParticipantEntity, ReadUserEntity)>(connection)
        .expect("Error loading participants");
    let inserted_trip = trip
        .filter(trip_id.eq(data_trip_id))
        .first::<ReadTripEntity>(connection)
        .expect("Error loading trip");
    let inserted_payer = user
        .filter(user_id.eq(data_payer_id))
        .first::<ReadUserEntity>(connection)
        .expect("Error loading payer");

    let transaction_dto = TransactionResponseDTO {
        id: inserted_transaction.id,
        name: inserted_transaction.name.clone(),
        value: inserted_transaction.value,
        description: inserted_transaction.description.clone(),
        payer: UserResponseDTO {
            id: inserted_payer.id,
            name: inserted_payer.name
        },
        trip: TripResponseDTO {
            id: inserted_trip.id,
            name: inserted_trip.name,
            description: inserted_trip.description
        },
        participants: inserted_participants_transaction.into_iter().map(|(_, u)| UserResponseDTO {
            id: u.id,
            name: u.name
        }).collect()
    };
    Json(transaction_dto)
}

#[put("/", data = "<data_transaction>")]
pub fn update_without_id(data_transaction: Json<TransactionRequestDTO>) -> Json<TransactionResponseDTO> {
    update(None, data_transaction)
}
#[put("/<param_id>", data = "<data_transaction>")]
pub fn update(param_id: Option<i64>, data_transaction: Json<TransactionRequestDTO>) -> Json<TransactionResponseDTO> {
    let connection = &mut establish_connection();

    let transaction_id = param_id.unwrap_or_else(|| data_transaction.id.expect("Id is required"));

    let current_transaction = transaction
        .filter(id.eq(transaction_id))
        .first::<ReadTransactionEntity>(connection)
        .expect("Error loading transaction");
    let current_participants_transaction = ReadTransactionParticipantEntity::belonging_to(&current_transaction)
        .inner_join(user)
        .load::<(ReadTransactionParticipantEntity, ReadUserEntity)>(connection)
        .expect("Error loading participants");

    let data_name = data_transaction.name.clone().unwrap_or(current_transaction.name.clone());
    let data_description = data_transaction.description.clone().unwrap_or(current_transaction.description.clone());
    let data_value = data_transaction.value.unwrap_or(current_transaction.value);
    let data_payer_id = data_transaction.payer_id.unwrap_or(current_transaction.payer_id);
    let data_trip_id = data_transaction.trip_id.unwrap_or(current_transaction.trip_id);
    let data_participants_id = data_transaction.participants_id.clone().unwrap_or(current_participants_transaction.into_iter().map(|(p, _)| p.user_id).collect());

    let updated_transaction: std::result::Result<ReadTransactionEntity, Error> = connection.transaction(|connection| {
        let updated_transaction = diesel::update(transaction.filter(id.eq(transaction_id)))
            .set((
                name.eq(data_name),
                description.eq(data_description),
                value.eq(data_value),
                tripId.eq(data_trip_id),
                payerId.eq(data_payer_id)
            ))
            .get_result::<ReadTransactionEntity>(connection)
            .expect("Error updating transaction");
        let participants = data_participants_id.clone().into_iter().map(|pid| UpdateTransactionParticipantEntity {
            transaction_id: Some(updated_transaction.id),
            user_id: Some(pid)
        }).collect::<Vec<UpdateTransactionParticipantEntity>>();
        diesel::delete(transaction_participants_user.filter(transaction_participants_user_transaction_id.eq(updated_transaction.id)))
            .execute(connection)
            .expect("Error deleting transaction participants");
        diesel::insert_into(transaction_participants_user)
            .values(&participants)
            .execute(connection)
            .expect("Error saving new transaction participants");
        Ok(updated_transaction)
    });
    let updated_transaction = updated_transaction.expect("Error updating transaction");
    let updated_participants_transaction = ReadTransactionParticipantEntity::belonging_to(&updated_transaction)
        .inner_join(user)
        .load::<(ReadTransactionParticipantEntity, ReadUserEntity)>(connection)
        .expect("Error loading participants");
    let updated_trip = trip
        .filter(trip_id.eq(data_trip_id))
        .first::<ReadTripEntity>(connection)
        .expect("Error loading trip");
    let updated_payer = user
        .filter(user_id.eq(data_payer_id))
        .first::<ReadUserEntity>(connection)
        .expect("Error loading payer");

    let transaction_dto = TransactionResponseDTO {
        id: updated_transaction.id,
        name: updated_transaction.name.clone(),
        value: updated_transaction.value,
        description: updated_transaction.description.clone(),
        payer: UserResponseDTO {
            id: updated_payer.id,
            name: updated_payer.name
        },
        trip: TripResponseDTO {
            id: updated_trip.id,
            name: updated_trip.name,
            description: updated_trip.description
        },
        participants: updated_participants_transaction.into_iter().map(|(_, u)| UserResponseDTO {
            id: u.id,
            name: u.name
        }).collect()
    };
    Json(transaction_dto)
}