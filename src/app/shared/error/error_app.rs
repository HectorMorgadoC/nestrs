pub mod error_app {
    use actix_web::{HttpResponse, ResponseError};
    use std::fmt;

    #[derive(Debug)]
    pub enum AppError {
        DatabaseError(String),
        ValidationError(String),
        NotFound(String),
        BusinessLogic(String),
        Unauthorized,
        InternalError(String),
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
                AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
                AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
                AppError::BusinessLogic(msg) => write!(f, "Business logic error: {}", msg),
                AppError::Unauthorized => write!(f, "Unauthorized"),
                AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            }
        }
    }
    
    impl ResponseError for AppError {
        fn error_response(&self) -> HttpResponse {
            match self {
                AppError::ValidationError(msg) => HttpResponse::BadRequest().json(msg),
                AppError::NotFound(msg) => HttpResponse::NotFound().json(msg),
                AppError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
                AppError::BusinessLogic(msg) => HttpResponse::UnprocessableEntity().json(msg),
                _ => HttpResponse::InternalServerError().json("Internal server error"),
            }
        }
    }

}