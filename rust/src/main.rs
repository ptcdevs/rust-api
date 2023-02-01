use std::{error::Error, net::Ipv4Addr};

use actix_web::{App, get, HttpResponse, HttpServer, middleware::Logger, post, Responder, web};
use actix_web::dev::JsonBody;
use actix_web::middleware::ErrorHandlerResponse::Response;
use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, OpenApi};
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

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Hello world!"),
        (status = NOT_FOUND, description = "not found!")
    )
)]
#[get("/")]
pub async fn hello() -> impl Responder {
                                     HttpResponse::Ok().body("Hello world!")
                                                                            }

#[utoipa::path(
    post,
    path = "/echo",
    request_body = HelloBlob,
    responses(
        (status = 200,
            description = "hello blob received",
            body = ResponseBlob,
            content_type = "application/json",
            example = json!({"id": 1123, "value": "test-value"})),
        (status = "5XX", description = "server error")
    )
)]
#[post("/echo")]
async fn echo(hello_blob: web::Json<RequestBlob>) -> actix_web::Result<impl Responder> {
    let response_blob = ResponseBlob {
        id: hello_blob.id,
        value: hello_blob.value.to_string()
    };
    Ok(web::Json(response_blob))
}

async fn manual_hello() -> impl Responder {
                                            HttpResponse::Ok().body("Hey there!")
    }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(paths(hello, echo), components(schemas(RequestBlob, ResponseBlob)))]
    struct ApiDoc;

    // println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

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

