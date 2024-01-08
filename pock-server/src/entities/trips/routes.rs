use diesel::prelude::*;
pub mod models;
use crate::trips::models::*;

use pock_server::establish_connection;
use pock_server::schema::trip::dsl::*;
use rocket::serde::json::{Json};


#[get("/")]
pub fn list() -> Json<Vec<TripResponseDTO>> {

    let connection = &mut establish_connection();
    let read_trips: Vec<ReadTripEntity> = trip
        .limit(5)
        .select(ReadTripEntity::as_select())
        .load(connection)
        .expect("Error loading trips!");
    let dto_trips: Vec<TripResponseDTO> = read_trips
        .into_iter()
        .map(|ctrip| TripResponseDTO {
            id: ctrip.id,
            name: ctrip.name,
            description: ctrip.description,
        })
        .collect();
    Json(dto_trips)
}

#[get("/<param_id>")]
pub fn read(param_id: i64) -> Json<TripResponseDTO> {

    let connection = &mut establish_connection();
    let read_trip: ReadTripEntity = trip
        .select(ReadTripEntity::as_select())
        .filter(id.eq(param_id))
        .first::<ReadTripEntity>(connection)
        .expect("Error loading trips!");
    let dto_trip: TripResponseDTO = TripResponseDTO {
        id: read_trip.id,
        name: read_trip.name,
        description: read_trip.description,
    };
    Json(dto_trip)
}

#[post("/", data = "<trip_data>")]
pub fn create(trip_data: Json<TripRequestDTO>) -> Json<TripResponseDTO> {
    let connection = &mut establish_connection();

    // Insert the new trip into the database
    let inserted_trip: ReadTripEntity = diesel::insert_into(trip)
        .values(UpdateTripEntity {
            id: None,
            name: trip_data.name.clone(),
            description: Some(trip_data.description.clone().unwrap_or(trip_data.name.clone())),
        })
        .get_result(connection)
        .expect("Error creating new trip");
    let dto_trip: TripResponseDTO = TripResponseDTO {
        id: inserted_trip.id,
        name: inserted_trip.name,
        description: inserted_trip.description,
    };
    Json(dto_trip)
}

#[put("/", data = "<trip_data>")]
pub fn update_without_id(trip_data: Json<TripRequestDTO>) -> Json<TripResponseDTO> {
    update(None, trip_data)
}
#[put("/<param_id>", data = "<trip_data>")]
pub fn update(param_id: Option<i64>, trip_data: Json<TripRequestDTO>) -> Json<TripResponseDTO> {

    let connection = &mut establish_connection();
    let actual_id = param_id.unwrap_or_else(|| trip_data.id.expect("Error updating trip, missing id in path or body."));

    // update trip in the database
    let inserted_trip: ReadTripEntity = diesel::update(trip)
        .filter(id.eq(actual_id))
        .set(UpdateTripEntity {
            id: Some(actual_id),
            name: trip_data.name.clone(),
            description: trip_data.description.clone(),
        })
        .get_result(connection)
        .expect("Error updating trip");

    let dto_trip: TripResponseDTO = TripResponseDTO {
        id: inserted_trip.id,
        name: inserted_trip.name,
        description: inserted_trip.description,
    };
    Json(dto_trip)
}

#[delete("/<param_id>")]
pub fn delete(param_id: i64) -> Json<TripResponseDTO> {

    let connection = &mut establish_connection();

    // delete trip in the database
    let deleted_trip: ReadTripEntity = diesel::delete(trip)
        .filter(id.eq(param_id))
        .get_result(connection)
        .expect("Error deleting trip");
    let dto_trip: TripResponseDTO = TripResponseDTO {
        id: deleted_trip.id,
        name: deleted_trip.name,
        description: deleted_trip.description,
    };
    Json(dto_trip)
}
