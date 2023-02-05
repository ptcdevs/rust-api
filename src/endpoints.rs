use actix_web::{Error, get, HttpResponse, Responder, web};
use actix_session::Session;
use actix_web::error::ErrorInternalServerError;
use actix_web::web::Redirect;
use actix_web::http::StatusCode;
use crate::error::MyError::MissingStateError;
use crate::github_oauth::github_oauth::{CallbackParams, GithubOauthFunctions};

