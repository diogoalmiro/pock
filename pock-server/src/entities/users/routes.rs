use diesel::prelude::*;
use pock_server::models::*;
use pock_server::*;
use rocket::serde::json::{json, Json, Value};

#[get("/")]
pub fn list() -> Value {
    use self::schema::user::dsl::*;

    let connection = &mut establish_connection();
    json!(user
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users!"))
}

#[get("/<param_id>")]
pub fn read(param_id: i64) -> Value {
    use self::schema::user::dsl::*;

    let connection = &mut establish_connection();
    json!(user
        .select(User::as_select())
        .filter(id.eq(param_id))
        .first::<User>(connection)
        .expect("Error loading post!"))
}

#[post("/", data = "<user_data>")]
pub fn create(user_data: Json<NewUser>) -> Value {
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
pub fn update(param_id: i64, user_data: Json<NewUser>) -> Value {
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
pub fn delete(param_id: i64) -> Value {
    use self::schema::user::dsl::*;

    let connection = &mut establish_connection();

    // delete user in the database
    let deleted_user: User = diesel::delete(user)
        .filter(id.eq(param_id))
        .get_result(connection)
        .expect("Error deleting user");

    json!(deleted_user)
}
