mod github_oauth;

use actix_web::cookie::Key;
use actix_web::dev::JsonBody;
use actix_web::middleware::ErrorHandlerResponse::Response;
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder, error, Error, middleware::ErrorHandlerResponse};
use serde::{Deserialize, Serialize};
use actix_web::web::Redirect;
use reqwest::StatusCode;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use crate::github_oauth::github_oauth::{GithubOauthConfig, login};

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
async fn echo(hello_blob: web::Json<RequestBlob>) -> actix_web::Result<impl Responder> {
    let response_blob = ResponseBlob {
        id: hello_blob.id,
        value: hello_blob.value.to_string(),
    };
    Ok(web::Json(response_blob))
}

#[utoipa::path(get, path = "/hey", responses(
(status = 200, description = "ok"), (status = NOT_FOUND, description = "not found!")))]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[utoipa::path(get, path = "/login", responses(
(status = FOUND, description = "found"),
(status = 5XX, description = "server error")))]
#[get("/login")]
pub async fn login() -> actix_web::Result<impl Responder, Error> {
    let github_authorize = "https://github.com/login/oauth/authorize?client_id=92e48c903bf0b3e8c4f3&redirect_uri=http%3A%2F%2Flocalhost%3A8080%2Fauth%2Fcallback&scope=user+repo&state=fo1Ooc1uofoozeithimah4iaW&allow_signup=false".to_string();
    //TODO: initialize github_oauth and retrieve this url from there
    //TODO: save state
    Ok(Redirect::to(github_authorize).using_status_code(StatusCode::FOUND))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(paths(hello, echo, manual_hello, login), components(schemas(RequestBlob, ResponseBlob)))]
    struct ApiDoc;

    //TODO: setup dependency injection using traits:
    // https://medium.com/geekculture/dependency-injection-in-rust-3822bf689888

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(login)
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
