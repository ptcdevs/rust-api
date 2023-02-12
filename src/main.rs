mod config;
mod error;
mod github_api;
mod hello_world;
mod tests;

use actix_session::config::{PersistentSession, CookieContentSecurity};
use actix_session::{Session, SessionGetError, SessionInsertError, SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::{self, Key};
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Redirect;
use actix_web::{get, Error, App, HttpResponse, HttpServer, Responder, web};
use config::AppConfig;
use crate::error::MyError::{EmptyTokenError, SessionError, UnauthorizedError};
use crate::github_api::config::config::{CallbackParams, GithubConfig, GithubOauthFunctions};
use error::MyError::MissingStateError;
use reqwest::StatusCode;
use std::env;
use std::sync::Arc;
use futures::TryFutureExt;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::error::MyError;

#[utoipa::path(get, path = "/login", responses(
(status = FOUND, description = "found"),
(status = 5XX, description = "server error")))]
#[get("/login")]
pub async fn login(session: Session, github_oauth: web::Data<dyn GithubOauthFunctions>) -> actix_web::Result<impl Responder, Error> {
    let github_authorize_url = github_oauth
        .get_authorize_url();
    session
        .insert("state", github_authorize_url.1)
        .map_err(|e| { ErrorInternalServerError(e) })?;

    Ok(Redirect::to(github_authorize_url.0).using_status_code(StatusCode::FOUND))
}

// on receiving callback:
// * extract state from session (and remove)
// * check that state matches session
// * if match, fetch access token (via github client); else throw error
// * insert access token to session
// * redirect to where they were going (pull redirect url from session, if exists); return 200 if not
#[utoipa::path(get, path = "/callback", responses(
(status = OK, description = "ok"),
(status = 5XX, description = "server error")))]
#[get("/callback")]
pub async fn callback(query: web::Query<CallbackParams>, session: Session, github_oauth: web::Data<dyn GithubOauthFunctions>) -> actix_web::Result<impl Responder, Error> {
    let session_state = session.get::<String>("state")
        .unwrap_or(None)
        .ok_or_else(|| MissingStateError)?;

    let callback_params = query.into_inner();
    let session_state_notempty = !session_state.is_empty();
    let session_state_match = session_state.eq(&callback_params.state);
    let client = if !session_state.is_empty() && session_state.eq(&callback_params.state) {
        let client = github_oauth
            .get_client(&callback_params.code)
            .await?;
        Ok(client)
    } else {
        Err(EmptyTokenError)
    }?;
    // would use this if async closures weren't unstable
    // let client = (!session_state.is_empty() && session_state.eq(&callback_params.state))
    //     .then(async || {
    //         github_oauth
    //             .get_client(&callback_params.code)
    //             .await?
    //     })
    //     .ok_or(EmptyTokenError);

    //TODO: match scopes
    session
        .insert("access_token", client.token.clone())
        .map_err(|e| SessionError)?;

    let redirect = session.get::<String>("redirect_url")
        .unwrap_or_else(|_| None)
        .map(|url| HttpResponse::Found().insert_header(("Location", url)).finish())
        .unwrap_or(HttpResponse::Ok().body(format!("success; access token: {}", client.token)));
    Ok(redirect)
    //alternative way to write the above statement
    // let redirect_url = session.get::<String>("redirect_url")
    //     .unwrap_or_else(|_| None);
    //
    // let response = Ok(match redirect_url {
    //     Some(redirect_url) => {
    //         HttpResponse::Found().insert_header(("Location", redirect_url)).finish()
    //     }
    //     None => HttpResponse::Ok().body(format!("success; access token: {}", client.token))
    // });
    // response
}

#[utoipa::path(get, path = "/commits", responses(
(status = OK, description = "ok"),
(status = 5XX, description = "server error")))]
#[get("/commits")]
pub async fn commits(session: Session, github_oauth: web::Data<dyn GithubOauthFunctions>) -> actix_web::Result<impl Responder, Error> {
    let access_token = session.get::<String>("access_token")
        .unwrap_or_else(|_| None)
        .ok_or_else(|| {
            let session_insert = session
                .insert("redirect_url", "/commits");
            return match session_insert {
                Err(_) => SessionError,
                Ok(_) => UnauthorizedError
            };
        })?;

    //TODO: save redirect back to /commits somewhere in session
    session
        .insert("redirect_url", "/callback")
        .map_err(|e| { ErrorInternalServerError(e) })?;

    Ok(HttpResponse::Ok().body(format!("commits tba; access_token: {}", access_token)))
}


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    #[derive(OpenApi)]
    #[openapi(paths(hello_world::hello, hello_world::echo, hello_world::manual_hello), components(schemas(hello_world::RequestBlob, hello_world::ResponseBlob)))]
    struct HelloWorld;

    #[derive(OpenApi)]
    #[openapi(paths(login, callback))]
    struct RestApi;

    HttpServer::new(|| {
        let config: AppConfig = confy::load_path("config/local/config.yaml").expect("failure reading github creds");
        let github_secret = env::var("GITHUB_OAUTH_CLIENT_SECRET").expect("missing github client secret from environment variables");
        let github_config = GithubConfig {
            client_id: config.github_oauth.client_id,
            client_secret: github_secret,
            redirect_url: config.github_oauth.redirect_url,
            scopes: config.github_oauth.scopes,
        };
        let arc_github_config: Arc<dyn GithubOauthFunctions> = Arc::new(github_config);
        //let github_config_data = web::Data::new(github_api);
        let github_config_data: web::Data<dyn GithubOauthFunctions> = web::Data::from(arc_github_config);

        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64])).cookie_content_security(CookieContentSecurity::Private)
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)))
                    .build())
            .service(hello_world::hello)
            .service(hello_world::echo)
            .route("/hey", web::get().to(hello_world::manual_hello))
            .service(login)
            .service(callback)
            .service(commits)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", HelloWorld::openapi()),
            )
            .app_data(github_config_data)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
