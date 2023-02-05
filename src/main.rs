mod github_oauth;
mod config;
mod error;

use std::borrow::Borrow;
use std::env;
use std::fmt::Formatter;
use std::sync::Arc;
use actix_session::{config::PersistentSession, Session, SessionInsertError, SessionMiddleware, storage::CookieSessionStore};
use actix_session::config::SessionLifecycle::BrowserSession;
use actix_session::config::{CookieContentSecurity, SessionMiddlewareBuilder, TtlExtensionPolicy};
use actix_web::cookie::{self, Key};
use actix_web::dev::JsonBody;
use actix_web::middleware::ErrorHandlerResponse::Response;
use actix_web::{App, Error, get, HttpRequest, HttpResponse, HttpServer, middleware::ErrorHandlerResponse, middleware::Logger, post, Responder, web};
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use actix_web::web::Redirect;
use reqwest::StatusCode;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use crate::github_oauth::github_oauth::{CallbackParams, GithubOauthConfig, GithubOauthConfigBorrowed};
use confy::{ConfyError, load_path};
use futures::future::err;
use config::AppConfig;
use rand::{distributions::Alphanumeric, Rng};
use error::MyError::MissingStateError;
// 0.8

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
pub async fn login(session: Session, github_oauth: web::Data<GithubOauthConfig>) -> actix_web::Result<impl Responder, Error> {
    let github_authorize_url = github_oauth.get_authorize_url();
    session
        .insert("state", github_authorize_url.1)
        .map_err(|e| { ErrorInternalServerError(e) })?;

    Ok(Redirect::to(github_authorize_url.0).using_status_code(StatusCode::FOUND))
}

#[utoipa::path(get, path = "/callback", responses(
(status = OK, description = "ok"),
(status = 5XX, description = "server error")))]
#[get("/callback")]
pub async fn callback(query: web::Query<CallbackParams>, session: Session, github_oauth: web::Data<GithubOauthConfig>) -> actix_web::Result<impl Responder, Error> {
    let session_state = session.get::<String>("state")
        .unwrap_or_else(|_| None)
        .ok_or_else(|| MissingStateError)?;
    let callback_params = query.into_inner();
    let access_token = if session_state.eq(&callback_params.state) {
        let access_token = github_oauth
            .get_access_token(callback_params.code)
            .await?;
        println!("access token: {}", access_token);
        Some(access_token)
    } else {
        None
    };

    //TODO: compare session state with query state
    // if match, take code and make a request against github api for access tokens
    // if no match, return new MyError::StateMismatch
    // if access token fetch succeeds, save token to user cookie
    // if access token fetch fails, return new MyError::TokenFetchFailure

    Ok(HttpResponse::Ok().body("callback success"))
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    #[derive(OpenApi)]
    #[openapi(paths(hello, echo, manual_hello, login, callback), components(schemas(RequestBlob, ResponseBlob)))]
    struct ApiDoc;

    HttpServer::new(|| {
        let config: AppConfig = confy::load_path("config/local/config.yaml").expect("failure reading github creds");
        let github_secret = env::var("GITHUB_OAUTH_CLIENT_SECRET").expect("missing github client secret from environment variables");
        let github_config = GithubOauthConfig {
            client_id: config.github_oauth.client_id,
            client_secret: github_secret,
            redirect_url: config.github_oauth.redirect_url,
            scopes: config.github_oauth.scopes,
        };

        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64])).cookie_content_security(CookieContentSecurity::Private)
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)))
                    .build())
            .service(hello)
            .service(echo)
            .service(login)
            .service(callback)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            .route("/hey", web::get().to(manual_hello))
            .app_data(web::Data::new(github_config))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
