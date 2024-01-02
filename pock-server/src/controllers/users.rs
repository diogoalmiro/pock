use pock_server::models::*;
use pock_server::*;
use diesel::prelude::*;
use rocket::Route;
use rocket::serde::json::{json, Value, Json};

#[get("/")]
fn list() -> Value {
    use self::schema::user::dsl::*;

    let connection = &mut establish_connection();
    json!(user
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts!"))
}

#[get("/<param_id>")]
fn read(param_id: i64) -> Value {
    use self::schema::user::dsl::*;

    let connection = &mut establish_connection();
    json!(user
        .select(User::as_select())
        .filter(id.eq(param_id))
        .first::<User>(connection)
        .expect("Error loading post!"))
}

#[post("/", data = "<user_data>")]
fn create(user_data: Json<NewUser>) -> Value {
    use self::schema::user::dsl::*;

    let connection = &mut establish_connection();

    // Insert the new user into the database
    let inserted_user: User = diesel::insert_into(user)
        .values(&*user_data)
        .get_result(connection)
        .expect("Error creating new user");

    json!(inserted_user)
}

#[put("/<param_id>", data = "<user_data>")]
fn update(param_id: i64, user_data: Json<NewUser>) -> Value {
    use self::schema::user::dsl::*;

    let connection = &mut establish_connection();

    // update user in the database
    let inserted_user: User = diesel::update(user)
        .filter(id.eq(param_id))
        .set(&*user_data)
        .get_result(connection)
        .expect("Error updating user");

    json!(inserted_user)
}

#[delete("/<param_id>")]
fn delete(param_id: i64) -> Value {
    use self::schema::user::dsl::*;

    let connection = &mut establish_connection();

    // delete user in the database
    let deleted_user: User = diesel::delete(user)
        .filter(id.eq(param_id))
        .get_result(connection)
        .expect("Error deleting user");

    json!(deleted_user)
}

pub fn get_routes() -> Vec<Route> {
    routes![list, read, create, update, delete]
}
