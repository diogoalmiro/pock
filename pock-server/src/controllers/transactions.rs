use diesel::prelude::*;
use pock_server::models::*;
use pock_server::*;
use rocket::serde::json::{json, Json, Value};
use rocket::Route;

#[get("/")]
fn list() -> Value {
    use self::schema::transaction::dsl::*;

    let connection = &mut establish_connection();
    json!(transaction
        .limit(5)
        .select(Transaction::as_select())
        .load(connection)
        .expect("Error loading transactions!"))
}

#[get("/<param_id>")]
fn read(param_id: i64) -> Value {
    use self::schema::transaction::dsl as transaction_dsl;
    //use self::schema::transaction_participants_user::dsl as participants_dsl;
    //use self::schema::trip::dsl as trip_dsl;
    //use self::schema::user::dsl as user_dsl;

    let connection = &mut establish_connection();
    let current_transaction = transaction_dsl::transaction
        .select(Transaction::as_select())
        .filter(transaction_dsl::id.eq(param_id))
        .first::<Transaction>(connection)
        .expect("Error loading transaction!");

    // dbg!(
    //     Trip::belonging_to(&current_transaction)
    //         .select(Trip::as_select())
    //         .first::<Trip>(connection)
    //         .expect("Error loading trip!")
    // );

    json!(dbg!(current_transaction))
}

#[post("/", data = "<transaction_data>")]
fn create(transaction_data: Json<TransactionRequestDTO>) -> Value {
    use self::schema::transaction::dsl::*;
    use self::schema::transaction_participants_user::dsl::*;

    let connection = &mut establish_connection();

    let new_transaction = NewTransaction {
        name: transaction_data.name.clone(),
        description: transaction_data.description.clone(),
        value: transaction_data.value,
        trip_id: transaction_data.trip_id,
        payer_id: transaction_data.payer_id,
    };

    // Insert the new transaction into the database
    let inserted_id: i64 = diesel::insert_into(transaction)
        .values(new_transaction)
        .returning(id)
        .get_result(connection)
        .expect("Error creating new transaction");

    json!({
        "id": inserted_id,
        "name": transaction_data.name.clone(),
        "description": transaction_data.description.clone(),
        "value": transaction_data.value.clone(),
        "trip_id": transaction_data.trip_id.clone(),
        "payer_id": transaction_data.payer_id.clone(),
        "participants_written": diesel::insert_into(transaction_participants_user)
        .values(
            transaction_data
                .participants
                .iter()
                .map(|participant_id| (transactionId.eq(inserted_id), userId.eq(participant_id)))
                .collect::<Vec<_>>(),
        )
        .execute(connection)
        .expect("Error creating new transaction participants")
    })
}
/*
#[put("/<param_id>", data = "<transaction_data>")]
fn update(param_id: i64, transaction_data: Json<TransactionRequestDTO>) -> Value {
    use self::schema::transaction::dsl::*;

    let connection = &mut establish_connection();

    // update transaction in the database
    let inserted_transaction: Transaction = diesel::update(transaction)
        .filter(id.eq(param_id))
        .set(&*transaction_data)
        .get_result(connection)
        .expect("Error updating transaction");

    json!(inserted_transaction)
}

#[delete("/<param_id>")]
fn delete(param_id: i64) -> Value {
    use self::schema::transaction::dsl::*;

    let connection = &mut establish_connection();

    // delete transaction in the database
    let deleted_transaction: Transaction = diesel::delete(transaction)
        .filter(id.eq(param_id))
        .get_result(connection)
        .expect("Error deleting transaction");

    json!(deleted_transaction)
}

pub fn get_routes() -> Vec<Route> {
    routes![list, read, create, update, delete]
}
*/

pub fn get_routes() -> Vec<Route> {
    routes![list, read, create]
}
