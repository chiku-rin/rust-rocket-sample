use api;
use rocket::launch;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", api::open_api())
        .mount("/", api::routes())
}
