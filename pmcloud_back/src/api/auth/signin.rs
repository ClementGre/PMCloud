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
use crate::database::user::{AuthToken, User};
use crate::utils::auth::DeviceInfo;
use crate::utils::errors_catcher::{ErrorResponder, ErrorResponse};
use crate::utils::utils::random_token;
use crate::utils::validation::validate_input;

#[derive(Deserialize, Debug)]
pub struct SigninData {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct SigninResponse {
    pub(crate) user_id: u32,
    pub(crate) auth_token: String,
}

#[post("/auth/signin", data = "<data>")]
pub fn auth_signin(data: Json<SigninData>, db: &rocket::State<DBPool>, device_info: DeviceInfo) -> Result<Json<SigninResponse>, ErrorResponder> {
    let conn: &mut DBConn = &mut db.get().unwrap();

    let user_opt = users::table
        .filter(users::dsl::email.eq(data.email.clone()))
        .select(User::as_select())
        .first::<User>(conn).optional().map_err(|e| {
        ErrorResponder::InternalError(Json(ErrorResponse {
            message: format!("An error occured: {}", e)
        }))
    })?;
    if let Some(user) = user_opt {
        if bcrypt::verify(data.password.clone(), &*user.password_hash) {
            return match user.status {
                UserStatus::Banned => {
                    Err(ErrorResponder::Unauthorized(Json(ErrorResponse {
                        message: "User is banned".to_string()
                    })))
                }
                UserStatus::Unconfirmed => {
                    Err(ErrorResponder::Unauthorized(Json(ErrorResponse {
                        message: "User is not confirmed".to_string()
                    })))
                }
                _ => {
                    let auth_token = AuthToken::insert_token_for_user(conn, user.id, device_info)?;

                    Ok(Json(SigninResponse {
                        user_id: user.id,
                        auth_token: hex::encode(auth_token),
                    }))
                }
            };
        }
    }
    Err(ErrorResponder::Unauthorized(Json(ErrorResponse {
        message: "Invalid email or password".to_string()
    })))
}
