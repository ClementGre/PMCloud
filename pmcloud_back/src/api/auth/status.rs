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
use crate::utils::auth::{DeviceInfo, UnauthenticatedUser};
use crate::utils::errors_catcher::{ErrorResponder, ErrorResponse};
use crate::utils::utils::random_token;
use crate::utils::validation::validate_input;

#[derive(Serialize, Debug)]
pub struct StatusResponse {
    pub(crate) status: u32,
}

#[get("/auth/status")]
pub fn auth_status(db: &rocket::State<DBPool>, user: UnauthenticatedUser) -> Result<Json<StatusResponse>, ErrorResponder> {

    Ok(Json(StatusResponse {
        status: user.user.id
    }))
}
