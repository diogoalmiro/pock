use diesel::prelude::*;
mod models;
use crate::users::models::*;

use pock_server::establish_connection;
use pock_server::schema::user::dsl::*;
use rocket::serde::json::{Json};

#[get("/")]
pub fn list() -> Json<Vec<UserResponseDTO>> {
    let connection = &mut establish_connection();
    let read_users: Vec<ReadUserEntity> = user
        .limit(5)
        .select(ReadUserEntity::as_select())
        .load(connection)
        .expect("Error loading users!");
    let dto_users: Vec<UserResponseDTO> = read_users
        .into_iter()
        .map(|cuser| UserResponseDTO {
            id: cuser.id,
            name: cuser.name,
        })
        .collect();
    Json(dto_users)
}

#[get("/<param_id>")]
pub fn read(param_id: i64) -> Json<UserResponseDTO> {
    let connection = &mut establish_connection();
    let read_user: ReadUserEntity = user
        .select(ReadUserEntity::as_select())
        .filter(id.eq(param_id))
        .first::<ReadUserEntity>(connection)
        .expect("Error loading users!");
    let dto_user: UserResponseDTO = UserResponseDTO {
        id: read_user.id,
        name: read_user.name,
    };
    Json(dto_user)
}

#[post("/", data = "<user_data>")]
pub fn create(user_data: Json<UserRequestDTO>) -> Json<UserResponseDTO> {
    let connection = &mut establish_connection();

    // Insert the new user into the database
    let inserted_user: ReadUserEntity = diesel::insert_into(user)
        .values(UpdateUserEntity {
            id: None,
            name: user_data.name.clone(),
        })
        .get_result(connection)
        .expect("Error creating new user");
    let dto_user: UserResponseDTO = UserResponseDTO {
        id: inserted_user.id,
        name: inserted_user.name,
    };
    Json(dto_user)
}

#[put("/", data = "<user_data>")]
pub fn update_without_id(user_data: Json<UserRequestDTO>) -> Json<UserResponseDTO> {
    update(None, user_data)
}
#[put("/<param_id>", data = "<user_data>")]
pub fn update(param_id: Option<i64>, user_data: Json<UserRequestDTO>) -> Json<UserResponseDTO> {
    let connection = &mut establish_connection();
    let actual_id = param_id.unwrap_or_else(|| user_data.id.expect("Error updating user, misssing id in path or body!"));

    // update user in the database
    let inserted_user: ReadUserEntity = diesel::update(user)
        .filter(id.eq(actual_id))
        .set(UpdateUserEntity {
            id: Some(actual_id),
            name: user_data.name.clone(),
        })
        .get_result(connection)
        .expect("Error updating user");
    let dto_user: UserResponseDTO = UserResponseDTO {
        id: inserted_user.id,
        name: inserted_user.name,
    };
    Json(dto_user)
}

#[delete("/<param_id>")]
pub fn delete(param_id: i64) -> Json<UserResponseDTO> {
    let connection = &mut establish_connection();

    // delete user in the database
    let deleted_user: ReadUserEntity = diesel::delete(user)
        .filter(id.eq(param_id))
        .get_result(connection)
        .expect("Error deleting user");
    let dto_user: UserResponseDTO = UserResponseDTO {
        id: deleted_user.id,
        name: deleted_user.name,
    };
    Json(dto_user)
}
