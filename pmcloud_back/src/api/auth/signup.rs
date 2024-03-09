use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{Deserialize, json::Json};
use rocket::serde::json::json;
use serde::Serialize;
use validator::{Validate, ValidateDoesNotContain, ValidationError, ValidationErrors};

use crate::database::database::DBPool;
use crate::utils::errors_catcher::{ErrorResponder, ErrorResponse};

#[derive(Deserialize, Debug, Validate)]
pub struct SignupData {
    #[validate(length(min = 3, max = 100, message = "Length must be between 3 and 100 characters"))]
    name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 100, message = "Length must be between 8 and 100 characters"))]
    password: String,
}

#[derive(Serialize, Debug)]
pub struct SignupResponse {
    user_id: u32,
}

#[get("/auth/signup", data = "<data>")]
pub fn auth_signup(data: Json<SignupData>, db: &rocket::State<DBPool>) -> Result<Json<SignupResponse>, ErrorResponder> {

    validate_input(data.into_inner())?;

    // If validation passes, proceed with your logic
    Ok(Json(SignupResponse {
        user_id: 1
    }))
}
