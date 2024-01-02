#[macro_use] extern crate rocket;

mod controllers {
    pub mod trips;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/trips", controllers::trips::get_controller())
}
