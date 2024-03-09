use rocket::Request;
use rocket::response::status;
use rocket::serde::json::Json;
use serde::Serialize;

use crate::utils::validation::ValidationErrorResponse;

#[derive(Responder)]
pub enum ErrorResponder {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorResponse>),
    #[response(status = 401, content_type = "json")]
    Unauthorized(Json<ErrorResponse>),
    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorResponse>),
    #[response(status = 422, content_type = "json")]
    UnprocessableEntity(Json<ErrorResponse>),
    #[response(status = 422, content_type = "json")]
    UnprocessableEntityFields(Json<ValidationErrorResponse>),
    #[response(status = 500, content_type = "json")]
    InternalError(Json<ErrorResponse>),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    /// Error message that the user will see
    pub message: String,
}

#[catch(400)]
pub fn bad_request() -> ErrorResponder {
    ErrorResponder::BadRequest(Json(ErrorResponse {
        message: "Bad request".to_string()
    }))
}

#[catch(401)]
pub fn unauthorized() -> ErrorResponder {
    ErrorResponder::Unauthorized(Json(ErrorResponse {
        message: "Unauthorized".to_string()
    }))
}
#[catch(404)]
pub fn not_found(req: &Request) -> ErrorResponder {
    ErrorResponder::NotFound(Json(ErrorResponse {
        message: format!("{} not found", req.uri())
    }))
}

/// When a json value type is incorrect
#[catch(422)]
pub fn unprocessable_entity() -> ErrorResponder {
    ErrorResponder::UnprocessableEntity(Json(ErrorResponse {
        message: "Unprocessable entity".to_string()
    }))
}
#[catch(500)]
pub fn internal_error() -> ErrorResponder {
    ErrorResponder::InternalError(Json(ErrorResponse {
        message: "Internal error".to_string()
    }))
}








