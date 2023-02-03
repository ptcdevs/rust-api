mod github_oauth;
mod new;

use std::borrow::Borrow;
use std::env;
use actix_web::cookie::Key;
use actix_web::dev::JsonBody;
use actix_web::middleware::ErrorHandlerResponse::Response;
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder, error, Error, middleware::ErrorHandlerResponse};
use serde::{Deserialize, Serialize};
use actix_web::web::Redirect;
use reqwest::StatusCode;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use crate::github_oauth::github_oauth::GithubOauthConfig;
use confy::{ConfyError, load_path};

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
pub async fn login(github_oauth: web::Data<GithubOauthConfig>) -> actix_web::Result<impl Responder, Error> {
    let github_authorize = github_oauth.get_authorize_url();
    //TODO: pass state to user session, then extract on /callback
    Ok(Redirect::to(github_authorize.0).using_status_code(StatusCode::FOUND))
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct AppConfig {
    github_oauth: GithubOauth,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct GithubOauth {
    app_name: String,
    app_url: String,
    client_id: String,
    redirect_url: String,
    scopes: Vec<String>,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    #[derive(OpenApi)]
    #[openapi(paths(hello, echo, manual_hello, login), components(schemas(RequestBlob, ResponseBlob)))]
    struct ApiDoc;

    let config: AppConfig = confy::load_path("config/local/config.yaml").expect("failure reading github creds");
    let github_secret = env::var("GITHUB_OAUTH_CLIENT_SECRET").expect("missing github client secret from environment variables");

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .service(login)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .route("/hey", web::get().to(manual_hello))
            .app_data(web::Data::new(GithubOauthConfig {
                client_id: config.github_oauth.client_id.to_string(),
                client_secret: github_secret.clone(),
                redirect_url: config.github_oauth.redirect_url.to_string(),
                scopes: config.github_oauth.scopes.to_vec(),
            }))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
