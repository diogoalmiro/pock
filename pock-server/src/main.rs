#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[launch]
fn rocket() -> _ {
    let builder = rocket::build();
    setup_routes(builder).mount("/", FileServer::from("/var/www/html"))
}
