use thiserror::Error;
use actix_web::{HttpResponse, ResponseError};
use jsonwebtoken::errors::{Error as JwtError};


#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Token missing")]
    TokenMissing,

    #[error("Token invalid")]
    TokenInvalid,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("An internal error occurred. Please try again later.")]
    InternalError,
    #[error("JWT error: {0}")]
    JwtError(#[from] JwtError),

}


impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AuthError::InvalidCredentials => HttpResponse::Unauthorized().json("Invalid credentials"),
            AuthError::TokenMissing => HttpResponse::BadRequest().json("No token provided"),
            AuthError::TokenInvalid => HttpResponse::Unauthorized().json("Invalid token"),
            AuthError::TokenExpired => HttpResponse::Unauthorized().json("Expired token"),
            AuthError::InternalError => HttpResponse::InternalServerError().json("Internal server error"),
			AuthError::JwtError(_) => HttpResponse::InternalServerError().json("Internal server error"),
		}
	}
}
