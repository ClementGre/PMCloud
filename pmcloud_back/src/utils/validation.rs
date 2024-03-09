use rocket::serde::json::Json;
use rocket::serde::Serialize;
use validator::{Validate, ValidationErrors};

use crate::utils::errors_catcher::ErrorResponder;

#[derive(Serialize, Debug)]
pub struct ValidationErrorResponse {
    message: String,
    errors: Vec<ValidationErrorFieldResponse>,
}

#[derive(Serialize, Debug)]
pub struct ValidationErrorFieldResponse {
    field: String,
    errors: Vec<String>,
}

impl From<ValidationErrors> for ValidationErrorResponse {
    fn from(errors: ValidationErrors) -> Self {
        ValidationErrorResponse {
            message: "Validation error".to_string(),
            errors: errors.field_errors().iter().map(|(field, errors)| {
                ValidationErrorFieldResponse {
                    field: field.to_string(),
                    errors: errors.iter().filter_map(|error| error.clone().message.map(|s| s.to_string())).collect(),
                }
            }).collect()
        }
    }
}

pub fn validate_input<T: Validate>(data: T) -> Result<(), ErrorResponder> {
    if let Err(errors) = data.validate() {
        return Err(ErrorResponder::UnprocessableEntityFields(Json(ValidationErrorResponse::from(errors))));
    }
    Ok(())
}
