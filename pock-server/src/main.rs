#[macro_use] extern crate rocket;

mod controllers {
    pub mod trips;
    pub mod users;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/trips", controllers::trips::get_routes())
        .mount("/users", controllers::users::get_routes())
}
