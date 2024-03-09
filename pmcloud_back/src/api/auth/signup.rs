use diesel::{insert_into, RunQueryDsl};
use pwhash::bcrypt;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{Deserialize, json::Json};
use rocket::serde::json::json;
use serde::Serialize;
use time::{OffsetDateTime, PrimitiveDateTime};
use validator::{Validate, ValidateDoesNotContain, ValidationError, ValidationErrors};

use crate::database::database::DBPool;
use crate::database::schema::{UserConfirmAction, users, UserStatus};
use crate::database::user::User;
use crate::utils::errors_catcher::{ErrorResponder, ErrorResponse};
use crate::utils::validation::validate_input;
use rand::{RngCore, rngs::OsRng};

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
    pub(crate) user_id: u32,
    pub(crate) auth_token: String,
}

#[get("/auth/signup", data = "<data>")]
pub fn auth_signup(data: Json<SignupData>, db: &rocket::State<DBPool>, user: Option<User>) -> Result<Json<SignupResponse>, ErrorResponder> {
    validate_input(&data)?;

    let conn = &mut db.get().unwrap();
    let now = OffsetDateTime::now_utc();

    let mut auth_token = vec![0u8; 16];
    OsRng.fill_bytes(&mut auth_token);

    if user.is_some() {
        println!("User: {:?}", user);
        return Err(ErrorResponder::Unauthorized(Json(ErrorResponse {
            message: "User already signed in".to_string()
        })));
    }

    let user = User {
        id: 0,
        name: data.name.clone(),
        email: data.email.clone(),
        password_hash: bcrypt::hash(data.password.clone()).unwrap(),
        confirm_date: PrimitiveDateTime::new(now.date(), now.time()),
        confirm_action: UserConfirmAction::Signup,
        confirm_token: None,
        confirm_code: 0,
        confirm_code_trials: 0,
        auth_token: auth_token.clone(),
        status: UserStatus::Unconfirmed,
        storage_count_mo: 0,
    };

    insert_into(users::dsl::users)
        .values(&user)
        .execute(conn).map_err(|e| {
            ErrorResponder::InternalError(Json(ErrorResponse {
                message: format!("Failed to insert user: {}", e)
            }))
        })?;

    // If validation passes, proceed with your logic
    Ok(Json(SignupResponse {
        user_id: user.id,
        auth_token: hex::encode(auth_token),
    }))
}
