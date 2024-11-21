use rocket::launch;
use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from("."))
}