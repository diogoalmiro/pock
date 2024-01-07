#[macro_use]
extern crate rocket;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[launch]
fn rocket() -> _ {
    let builder = rocket::build();
    setup_routes(builder)
}
