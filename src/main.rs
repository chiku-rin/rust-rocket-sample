use api;
use rocket::launch;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", api::routes())
}
