use rocket::{get, routes};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

#[get("/hello")]
fn index(_a: middleware::GuardA) -> &'static str {
    "Hello, world!"
}

#[utoipa::path(
    context_path = "/",
    responses(
        (status = 200, description = "health check", body = String)
    )
)]
#[get("/health")]
fn health() -> &'static str {
    "OK"
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, health]
}
#[derive(OpenApi)]
#[openapi(paths(health))]
struct ApiDoc;

pub fn open_api() -> RapiDoc<'static, 'static, 'static> {
    RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/rapidoc")
}
