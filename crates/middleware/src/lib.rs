use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct GuardA;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GuardA {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("Authorization") {
            Some(val) if val == "Bearer test" => Outcome::Success(GuardA),
            _ => Outcome::Error((Status::Unauthorized, "Unauthorized".to_string())),
        }
    }
}
