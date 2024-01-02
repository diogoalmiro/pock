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

#[get("/<param_id>")]
fn read(param_id: i64) -> Value {
    use self::schema::trip::dsl::*;

    let connection = &mut establish_connection();
    json!(trip
        .select(Trip::as_select())
        .filter(id.eq(param_id))
        .first::<Trip>(connection)
        .expect("Error loading post!"))
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

#[put("/<param_id>", data = "<trip_data>")]
fn update(param_id: i64, trip_data: Json<NewTrip>) -> Value {
    use self::schema::trip::dsl::*;

    let connection = &mut establish_connection();

    // update trip in the database
    let inserted_trip: Trip = diesel::update(trip)
        .filter(id.eq(param_id))
        .set(&*trip_data)
        .get_result(connection)
        .expect("Error updating trip");

    json!(inserted_trip)
}

#[delete("/<param_id>")]
fn delete(param_id: i64) -> Value {
    use self::schema::trip::dsl::*;

    let connection = &mut establish_connection();

    // delete trip in the database
    let deleted_trip: Trip = diesel::delete(trip)
        .filter(id.eq(param_id))
        .get_result(connection)
        .expect("Error deleting trip");

    json!(deleted_trip)
}

pub fn get_controller() -> Vec<Route> {
    routes![read, list, create, update, delete]
} 