use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use sea_orm::DatabaseConnection;
use serde::Serialize;

pub mod tweets;
pub mod users;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadRequest,
}

#[derive(Serialize)]
struct ErrorBody {
    message: String,
}

impl ApiError {
    fn response_body(&self) -> ErrorBody {
        match *self {
            ApiError::InternalError => ErrorBody {
                message: String::from("Internal server error"),
            },
            ApiError::BadRequest => ErrorBody {
                message: String::from("Bad request"),
            },
        }
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.response_body())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}
