use pock_server::models::*;
use pock_server::*;
use diesel::prelude::*;
use rocket::Route;
use rocket::serde::json::{json, Value, Json};

#[get("/")]
fn list() -> Value {
    use self::schema::trip::dsl::*;

    let connection = &mut establish_connection();
    json!(trip
        .limit(5)
        .select(Trip::as_select())
        .load(connection)
        .expect("Error loading posts!"))
}

#[post("/", data = "<trip_data>")]
fn create(trip_data: Json<NewTrip>) -> Value {
    use self::schema::trip::dsl::*;

    let connection = &mut establish_connection();

    // Insert the new trip into the database
    let inserted_trip: Trip = diesel::insert_into(trip)
        .values(&*trip_data)
        .get_result(connection)
        .expect("Error creating new trip");

    json!(inserted_trip)
}

pub fn get_controller() -> Vec<Route> {
    routes![list, create]
} 