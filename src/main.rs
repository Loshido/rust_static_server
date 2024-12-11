use rocket::launch;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};

use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("."))
        .attach(CacheControlFairing)
}
pub struct CacheControlFairing;

#[rocket::async_trait]

impl Fairing for CacheControlFairing {
    fn info(&self) -> Info {
        Info {
            name: "Add Cache-Control header",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(rocket::http::Header::new("Cache-Control", "public, max-age=86400"));
    }
}