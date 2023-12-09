use rocket::{get, routes};

extern crate middleware;

#[get("/hello")]
fn index(_a: middleware::GuardA) -> &'static str {
    "Hello, world!"
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, health]
}
