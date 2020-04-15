use actix_web::error::Error as ActixError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct DataError {
    pub status_code: u16,
    pub message: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    pub status_code: u16,
    pub data: DataError,
}

impl ApiError {
    pub fn new<T: Into<String>>(status_code: u16, message: T, path: String) -> ApiError {
        ApiError {
            status_code,
            data: DataError {
                status_code,
                message: message.into(),
                path,
            },
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.data.message.as_str())
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::DatabaseError(_, err) => ApiError::new(
                409,
                &err.message().to_string(),
                match err.constraint_name() {
                    None => match err.table_name() {
                        None => "generic".to_string(),
                        Some(error) => String::from(error),
                    },
                    Some(error) => String::from(error),
                },
            ),
            DieselError::NotFound => {
                ApiError::new(404, "Record not found".to_string(), "generic".to_string())
            }
            err => ApiError::new(500, format!("Diesel error: {}", err), "generic".to_string()),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.data.message.clone(),
            false => {
                error!("{}", self.data.message);
                "Internal server error".to_string()
            }
        };

        HttpResponse::build(status_code).json::<DataError>(DataError {
            message,
            status_code: status_code.into(),
            path: self.data.path.clone(),
        })
    }
}

impl From<ActixError> for ApiError {
    fn from(error: ActixError) -> ApiError {
        ApiError::new(500, error.to_string(), "generic".to_string())
    }
}
