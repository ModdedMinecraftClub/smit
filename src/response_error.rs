#![allow(unused)]

use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub type GenericResult<T> = Result<T, GenericError>;
pub type GenericResponse = GenericResult<HttpResponse>;

#[derive(Debug)]
pub enum GenericError {
    InternalServerError(Box<dyn Error>),
    ForbiddenError,
    BrokenInvariant(String),
    MalformedRequest(String),
    NotFoundError,
    Conflict(String),
    Custom(String, StatusCode),
}

impl<T: Error + 'static> From<T> for GenericError {
    fn from(err: T) -> Self {
        Self::InternalServerError(Box::new(err))
    }
}

impl ResponseError for GenericError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ForbiddenError => StatusCode::FORBIDDEN,
            Self::BrokenInvariant(_) => StatusCode::BAD_REQUEST,
            Self::MalformedRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFoundError => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Custom(_, code) => *code,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InternalServerError(error) => {
                println!("An error occurred when processing a request: {:?}", error);
                HttpResponseBuilder::new(self.status_code()).body("Internal Server Error")
            }
            Self::ForbiddenError => HttpResponseBuilder::new(self.status_code()).body("Forbidden"),
            Self::BrokenInvariant(message) => {
                println!(
                    "An invariant was broken when processing a request: {:?}",
                    message
                );
                HttpResponseBuilder::new(self.status_code()).body("Broken Invariant")
            }
            Self::MalformedRequest(message) => {
                HttpResponseBuilder::new(self.status_code()).body(message)
            }
            Self::NotFoundError => HttpResponseBuilder::new(self.status_code()).body("Not Found"),
            Self::Conflict(message) => HttpResponseBuilder::new(self.status_code()).body(message),
            Self::Custom(message, code) => HttpResponseBuilder::new(*code).body(message),
        }
    }
}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Generic Error")
    }
}

#[derive(Debug)]
pub struct StringError(pub String);

impl<'a> From<&'a str> for StringError {
    fn from(slice: &'a str) -> Self {
        StringError(String::from(slice))
    }
}

impl Error for StringError {}

impl Display for StringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
