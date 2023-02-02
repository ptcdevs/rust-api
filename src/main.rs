mod r#github_client;

use actix_web::cookie::Key;
use actix_web::dev::JsonBody;
use actix_web::middleware::ErrorHandlerResponse::Response;
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use actix_session::{Session, SessionMiddleware, storage::RedisActorSessionStore};
use serde::{Deserialize, Serialize};
use std::{error::Error, net::Ipv4Addr};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(ToSchema, Deserialize)]
struct RequestBlob {
    id: u64,
    value: String,
}

#[derive(ToSchema, Serialize)]
struct ResponseBlob {
    id: u64,
    value: String,
}

#[utoipa::path(get, path = "/",
    responses(
        (status = 200, description = "ok", content_type = "text/plain" ),
        (status = NOT_FOUND, description = "not found!")))]
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[utoipa::path(post, path = "/echo", request_body = RequestBlob,
    responses(
        (status = 200, description = "request blob received", body = ResponseBlob, content_type = "application/json",
            example = json ! ({"id": 1123, "value": "test-value"})),
        (status = "5XX", description = "server error")))]
#[post("/echo")]
async fn echo(hello_blob: web::Json<RequestBlob>) -> actix_web::Result<impl Responder> {
    let response_blob = ResponseBlob {
        id: hello_blob.id,
        value: hello_blob.value.to_string(),
    };
    Ok(web::Json(response_blob))
}

#[utoipa::path(get, path = "/hey",
responses((status = 200, description = "ok"), (status = NOT_FOUND, description = "not found!")))]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(OpenApi)]
#[openapi(paths(hello, echo), components(schemas(RequestBlob, ResponseBlob)))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
