use anyhow::anyhow;
use axum::extract::rejection::{JsonRejection, JsonDataError};
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use thiserror;

#[derive(Debug)]
pub struct Status {
    code: u16,
    message: String,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
    #[error("{0}")]
    Custom(Status),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    InternalServerError(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl Error {
    pub fn get_status(self) -> Status {
        return match self {
            Error::JsonRejection(err) => Status { code: 400u16, message: err.to_string() },
            Error::Custom(status) => status,
            Error::BadRequest(err) => Status { code: 400u16, message: err },
            Error::Unauthorized(err) => Status { code: 401u16, message: err },
            Error::Forbidden(err) => Status { code: 403u16, message: err },
            Error::NotFound(err) => Status { code: 404u16, message: err },
            Error::InternalServerError(err) => Status { code: 500u16, message: err },
            Error::Unknown(err) => {
                eprintln!("Error: {:?}", err);
                Status {
                    code: 500,
                    message: err.to_string(),
                }
            }
        };
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(e: sea_orm::DbErr) -> Self {
        Error::Unknown(anyhow!(e))
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = self.get_status();
        let status_code = match StatusCode::from_u16(status.code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        // let res: ApiResponse<()> = ApiResponse::fail(status.code, status.message);
        let data =
            Json(json!({"code": status.code, "message": status.message,"data": None::<String>}));
        (status_code, data).into_response()
    }
}
