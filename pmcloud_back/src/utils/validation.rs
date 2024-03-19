use rocket::serde::json::Json;
use validator::Validate;

use crate::utils::errors_catcher::{ErrorResponder, ErrorType};

pub fn validate_input<T: Validate>(data: &Json<T>) -> Result<(), ErrorResponder> {
    if let Err(errors) = data.validate() {
        let message = errors.field_errors().iter().map(|(field, errors)| {
            field.to_string() + ": " + &errors.iter().filter_map(|error| error.clone().message.map(|s| s.to_string())).collect::<Vec<String>>().join(", ")
        }).collect::<Vec<String>>().join(", ");

        return ErrorType::InvalidInput(message).to_err();
    }
    Ok(())
}
