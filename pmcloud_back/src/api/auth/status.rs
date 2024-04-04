use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, select, SelectableHelper, update};
use diesel::dsl::insert_into;
use pwhash::bcrypt;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use validator::Validate;
use crate::database::database::{DBConn, DBPool};
use crate::database::schema::auth_tokens;
use crate::database::schema::{inet6_aton, last_insert_id, UserStatus};
use crate::database::schema::users;
use crate::database::user::User;
use crate::utils::auth::{DeviceInfo};
use crate::utils::errors_catcher::{ErrorResponder, ErrorResponse, ErrorType};
use crate::utils::utils::random_token;
use crate::utils::validation::validate_input;

#[derive(Serialize, Debug)]
pub struct StatusResponse {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) status: UserStatus,
}

#[get("/auth/status")]
pub fn auth_status(user: Option<User>) -> Result<Json<StatusResponse>, ErrorResponder> {
    if let Some(user) = user {
        return Ok(Json(StatusResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            status: user.status,
        }));
    }
    Err(ErrorType::Unauthorized.to_responder())
}
