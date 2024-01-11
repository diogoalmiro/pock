use diesel::prelude::*;
use rocket::serde::json::{Json};
pub mod models;
use crate::reports::models::*;
use pock_server::schema::transaction::dsl::{transaction, tripId as transaction_trip_id};
use pock_server::schema::trip::dsl::{id as trip_id, trip};
use pock_server::schema::user::dsl::{id as user_id, user};
use pock_server::establish_connection;

use crate::trips::models::{ReadTripEntity, TripResponseDTO};
use crate::transactions::models::{ReadTransactionEntity, ReadTransactionParticipantEntity};
use crate::users::models::{ReadUserEntity, UserResponseDTO};

#[get("/<param_id>")]
pub fn get_reports(param_id: i64) -> Json<ReportResponseDTO> {
    let connection = &mut establish_connection();

    let current_trip = trip
        .filter(trip_id.eq(param_id))
        .first::<ReadTripEntity>(connection)
        .expect("Error loading trips!");
    let mut user_ids: Vec<i64> = Vec::new();
    let current_transactions = transaction
        .filter(transaction_trip_id.eq(param_id))
        .load::<ReadTransactionEntity>(connection)
        .expect("Error loading transactions");
    user_ids.extend(current_transactions.iter().map(|t| t.payer_id));

    let current_participants = ReadTransactionParticipantEntity::belonging_to(&current_transactions)
        .load::<ReadTransactionParticipantEntity>(connection)
        .expect("Error loading transactions")
        .grouped_by(&current_transactions);
    user_ids.extend(current_participants.iter().flatten().map(|t| t.user_id));
    
    let unique_users = user
        .filter(user_id.eq_any(user_ids.clone()))
        .load::<ReadUserEntity>(connection)
        .expect("Error loading users");
    let mut final_debt: Vec<ReportTransactionParticipantResponseDTO> = unique_users.iter().map(|u| ReportTransactionParticipantResponseDTO {
        id: u.id,
        name: u.name.clone(),
        value: 0.0,
    }).collect();
    
    let report_transactions_dto = current_transactions.iter().map(|ct| {
        let participants = current_participants.iter().find(|&p| p.iter().any(|t| t.transaction_id == ct.id));
        let size = participants.map(|p| p.len()).unwrap_or(0);
        let payer = unique_users.iter().find(|u| u.id == ct.payer_id).unwrap();
        let users_values: Vec<ReportTransactionParticipantResponseDTO> = unique_users.iter().map(|u| {
            let value;
            if u.id == ct.payer_id && participants.is_some() && participants.unwrap().iter().any(|p| p.user_id == u.id) {
                value = -ct.value + ct.value / size as f64;
            } else if u.id == ct.payer_id {
                value = -ct.value;
            } else if participants.is_some() && participants.unwrap().iter().any(|p| p.user_id == u.id) {
                value = ct.value / size as f64;
            }
            else {
                value = 0.0;
            }

            ReportTransactionParticipantResponseDTO {
                id: u.id,
                name: u.name.clone(),
                value: if value.is_normal() { value } else { 0.0 },
            }
        }).collect();
        final_debt.iter_mut().zip(users_values.iter()).for_each(|(f, u)| {
            f.value += u.value;
        });
        
        
        ReportTransactionResponseDTO {
            id: ct.id,
            name: ct.name.clone(),
            description: ct.description.clone(),
            value: ct.value,
            payer: UserResponseDTO {
                id: payer.id,
                name: payer.name.clone(),
            },
            participants: users_values,
        }
    }).collect();
    
    Json(ReportResponseDTO {
        trip: TripResponseDTO {
            id: current_trip.id,
            name: current_trip.name,
            description: current_trip.description,
        },
        transactions: report_transactions_dto,
        debts: final_debt,
    })
}