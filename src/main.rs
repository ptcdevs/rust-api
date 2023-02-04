mod github_oauth;
mod config;

use std::borrow::Borrow;
use std::env;
use std::fmt::Formatter;
use std::sync::Arc;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware, SessionInsertError};
use actix_session::config::SessionLifecycle::BrowserSession;
use actix_session::config::{CookieContentSecurity, SessionMiddlewareBuilder, TtlExtensionPolicy};
use actix_web::cookie::{self, Key};
use actix_web::dev::JsonBody;
use actix_web::middleware::ErrorHandlerResponse::Response;
use actix_web::{App, error, Error, get, HttpRequest, HttpResponse, HttpServer, middleware::ErrorHandlerResponse, middleware::Logger, post, Responder, web};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use actix_web::web::Redirect;
use reqwest::StatusCode;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use crate::github_oauth::github_oauth::{GithubOauthConfig, GithubOauthConfigBorrowed};
use confy::{ConfyError, load_path};
use futures::future::err;
use config::AppConfig;
use rand::{distributions::Alphanumeric, Rng};
// 0.8
use derive_more::{Display, Error};

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
    let _ = session
        .insert("state", github_authorize_url.1)
        .or_else(|error: SessionInsertError| {
            return Err(error::ErrorInternalServerError(error));
        });

    Ok(Redirect::to(github_authorize_url.0).using_status_code(StatusCode::FOUND))
}

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "session error")]
    SessionError,

    #[display(fmt = "missing state error")]
    MissingStateError,
}

impl actix_web::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::SessionError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::MissingStateError => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

#[utoipa::path(get, path = "/callback", responses(
(status = OK, description = "ok"),
(status = 5XX, description = "server error")))]
#[get("/callback")]
pub async fn callback(request: HttpRequest, session: Session, github_oauth: web::Data<GithubOauthConfig>) -> actix_web::Result<impl Responder, Error> {
    let state = session.get::<String>("stateff")
        .or_else(|error| {
            return Err(MyError::SessionError);
        })
        .unwrap()
        .ok_or_else(|| { MyError::MissingStateError })
        .or_else(|err| {
            return Err(err);
        });

    if(state.is_ok()) {
        return Ok(HttpResponse::Ok().body("Hello world!"))
    } else {
        return Err(Error::from(state.unwrap_err()));
    }
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
