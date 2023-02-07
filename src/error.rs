use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error, Clone)]
pub enum MyError {
    #[display(fmt = "session error")]
    SessionError,

    #[display(fmt = "state value missing or does not match session")]
    MissingStateError,

    #[display(fmt = "error requesting access token")]
    TokenResponseError,

    #[display(fmt = "error extracting access token response body")]
    TokenResponseBodyError,

    #[display(fmt = "error parsing access token from response body")]
    TokenResponseParseError,

    #[display(fmt = "no access token returned")]
    EmptyTokenError,
}

impl actix_web::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::SessionError => StatusCode::BAD_REQUEST,
            MyError::MissingStateError => StatusCode::BAD_REQUEST,
            MyError::TokenResponseError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::TokenResponseBodyError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::TokenResponseParseError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::EmptyTokenError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
