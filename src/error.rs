use std::error::Error;
use std::fmt::{Display, Formatter};
use std::panic::Location;
use actix_web::http::header::ContentType;
use actix_web::http::{header, StatusCode};
use actix_web::HttpResponse;
use actix_web::web::Redirect;
use derive_more::{Display, Error};
use crate::error::MyError::{EmptyTokenError, MissingStateError, SessionError, TokenResponseBodyError, TokenResponseError, TokenResponseParseError, UnauthorizedError};

// #[derive(Debug, Display, Error, Clone)]
// pub enum MyError {
//     #[display(fmt = "session error")]
//     SessionError,
//
//     #[display(fmt = "state value missing or does not match session")]
//     MissingStateError,
//
//     #[display(fmt = "error requesting access token")]
//     TokenResponseError,
//
//     #[display(fmt = "error extracting access token response body")]
//     TokenResponseBodyError,
//
//     #[display(fmt = "error parsing access token from response body")]
//     TokenResponseParseError,
//
//     #[display(fmt = "no access token returned")]
//     EmptyTokenError,
//
//     #[display(fmt = "unauthorized; redirecto to /login")]
//     UnauthorizedError,
//
//     #[display(fmt = "github api error")]
//     GithubApi(String),
// }
#[derive(Debug, Clone)]
pub enum MyError {
    SessionError,
    MissingStateError,
    TokenResponseError,
    TokenResponseBodyError,
    TokenResponseParseError,
    EmptyTokenError,
    UnauthorizedError,
    GithubApi(String),
}

impl Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &*self {
            TokenResponseError | TokenResponseBodyError | TokenResponseParseError |
            EmptyTokenError | MissingStateError | SessionError | UnauthorizedError =>
                write!(f, "{:?}", self),
            MyError::GithubApi(message, ..) => write!(f, "{:?}; {}", self, message),
        }
    }
}

impl actix_web::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::TokenResponseError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::TokenResponseBodyError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::TokenResponseParseError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::EmptyTokenError => StatusCode::INTERNAL_SERVER_ERROR,
            MissingStateError | SessionError | UnauthorizedError => StatusCode::UNAUTHORIZED,
            MyError::GithubApi(_, ..) => StatusCode::BAD_REQUEST
        }
    }

    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::UnauthorizedError => HttpResponse::Found()
                .insert_header(("Location", "/login"))
                .finish(),
            _ => HttpResponse::build(self.status_code())
                .insert_header(ContentType::html())
                .body(self.to_string())
        }
    }
}
