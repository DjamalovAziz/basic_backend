use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct BasicError {
    pub message: String,
    pub status_code: u16,
}

impl BasicError {
    pub fn not_found_error(message: String) -> Self {
        BasicError {
            message,
            status_code: 404,
        }
    }

    pub fn cannot_create_error(message: String) -> Self {
        BasicError {
            message,
            status_code: 409,
        }
    }

    pub fn server_error(message: String) -> Self {
        BasicError {
            message,
            status_code: 500,
        }
    }

    pub fn bad_request_error(message: String) -> Self {
        BasicError {
            message,
            status_code: 400,
        }
    }

    pub fn forbidden_error(message: String) -> Self {
        BasicError {
            message,
            status_code: 403,
        }
    }
}

impl From<surrealdb::Error> for BasicError {
    fn from(error: surrealdb::Error) -> Self {
        Self {
            message: error.to_string(),
            status_code: 400,
        }
    }
}

impl From<diesel::result::Error> for BasicError {
    fn from(error: diesel::result::Error) -> Self {
        Self {
            message: error.to_string(),
            status_code: 400,
        }
    }
}

impl From<Box<dyn std::error::Error>> for BasicError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self {
            message: error.to_string(),
            status_code: 400,
        }
    }
}

impl From<validator::ValidationErrors> for BasicError {
    fn from(error: validator::ValidationErrors) -> Self {
        Self {
            message: error.to_string(),
            status_code: 422,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for BasicError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        Self {
            message: error.to_string(),
            status_code: 403,
        }
    }
}

impl fmt::Display for BasicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ResponseError for BasicError {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}
