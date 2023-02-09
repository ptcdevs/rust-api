use serde::{Deserialize, Serialize};
use actix_web::{get, HttpResponse, post, Responder, web};
use utoipa::ToSchema;

#[derive(ToSchema, Deserialize)]
pub struct RequestBlob {
    id: u64,
    value: String,
}

#[derive(ToSchema, Serialize)]
pub struct ResponseBlob {
    id: u64,
    value: String,
}

#[utoipa::path(get, path = "/",
responses((status = 200, description = "ok", content_type = "text/plain" ),
(status = NOT_FOUND, description = "not found!")))]
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[utoipa::path(post, path = "/echo", request_body = RequestBlob, responses(
(status = 200, description = "request blob received", body = ResponseBlob, content_type = "application/json"),
(status = "5XX", description = "server error")))]
#[post("/echo")]
pub async fn echo(hello_blob: web::Json<RequestBlob>) -> actix_web::Result<impl Responder> {
    let response_blob = ResponseBlob {
        id: hello_blob.id,
        value: hello_blob.value.to_string(),
    };
    Ok(web::Json(response_blob))
}

#[utoipa::path(get, path = "/hey", responses(
(status = 200, description = "ok"), (status = NOT_FOUND, description = "not found!")))]
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
