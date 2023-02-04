use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum MyError {
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
