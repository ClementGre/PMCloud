use diesel::{ExpressionMethods, insert_into, QueryDsl, RunQueryDsl, select};
use diesel::associations::HasTable;
use diesel::dsl::Nullable;
use diesel::sql_types::Binary;
use pwhash::bcrypt;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{Deserialize, json::Json};
use rocket::serde::json::json;
use serde::Serialize;
use time::{OffsetDateTime, PrimitiveDateTime};
use validator::{Validate, ValidateDoesNotContain, ValidationError, ValidationErrors};

use crate::database::database::DBPool;
use crate::database::schema::{auth_tokens::dsl::*, inet6_aton, last_insert_id, UserConfirmAction, users::dsl::*, UserStatus};
use crate::database::user::User;
use crate::utils::auth::DeviceInfo;
use crate::utils::errors_catcher::{ErrorResponder, ErrorResponse};
use crate::utils::utils::random_token;
use crate::utils::validation::validate_input;

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
    pub(crate) session_id: u16,
    pub(crate) auth_token: String,
}

#[get("/auth/signup", data = "<data>")]
pub fn auth_signup(data: Json<SignupData>, db: &rocket::State<DBPool>, device_info: DeviceInfo) -> Result<Json<SignupResponse>, ErrorResponder> {
    // validate_input(&data)?;
    let conn = &mut db.get().unwrap();

    // if user.is_some() {
    //     println!("User: {:?}", user);
    //     return Err(ErrorResponder::Unauthorized(Json(ErrorResponse {
    //         message: "User already signed in".to_string()
    //     })));
    // }

    let result = insert_into(users)
        .values((
            name.eq(data.name.clone()),
            email.eq(data.email.clone()),
            password_hash.eq(bcrypt::hash(data.password.clone()).unwrap())
        ))
        .execute(conn).map_err(|e| {
        ErrorResponder::InternalError(Json(ErrorResponse {
            message: format!("Failed to insert user: {}", e)
        }))
    })?;
    if result == 0 {
        return Err(ErrorResponder::InternalError(Json(ErrorResponse {
            message: "Failed to insert user. An account with the same email might exist.".to_string()
        })));
    }
    let uid = select(last_insert_id()).get_result::<u64>(conn).map_err(|e| {
        ErrorResponder::InternalError(Json(ErrorResponse {
            message: format!("Failed to get last insert id: {}", e)
        }))
    })? as u32;

    // Inserting auth token

    println!("Device info: {:?}", device_info);

    let auth_token = random_token(32);
    let session_id = rand::random::<u16>();
    let result = insert_into(auth_tokens)
        .values((
            user_id.eq(uid),
            token.eq(auth_token.clone()),
            last_session_id.eq(session_id),
            user_agent.eq(device_info.user_agent),
            ip_address.eq(inet6_aton(device_info.ip_address))
        ))
        .execute(conn).map_err(|e| {
        ErrorResponder::InternalError(Json(ErrorResponse {
            message: format!("Failed to insert auth token: {}", e)
        }))
    })?;

    // TODO: Send confirmation email

    // If validation passes, proceed with your logic
    Ok(Json(SignupResponse {
        user_id: uid,
        session_id,
        auth_token: hex::encode(auth_token),
    }))
}
