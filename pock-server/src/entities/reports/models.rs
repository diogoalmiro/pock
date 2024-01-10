use rocket::serde::{Serialize};

use crate::trips::models::*;
use crate::users::models::*;


#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ReportResponseDTO {
    pub trip: TripResponseDTO,
    pub transactions: Vec<ReportTransactionResponseDTO>,
    pub debts: Vec<ReportTransactionParticipantResponseDTO>,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ReportTransactionResponseDTO {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub value: f64,
    pub payer: UserResponseDTO,
    pub participants: Vec<ReportTransactionParticipantResponseDTO>
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ReportTransactionParticipantResponseDTO {
    pub id: i64,
    pub name: String,
    pub value: f64,
}
