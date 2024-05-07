use diesel::{ExpressionMethods, insert_into, QueryDsl, RunQueryDsl, select};
use diesel::associations::HasTable;
use diesel::dsl::Nullable;
use diesel::result::DatabaseErrorKind;
use diesel::sql_types::Binary;
use pwhash::bcrypt;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{Deserialize, json::Json};
use rocket::serde::json::json;
use serde::Serialize;
use validator::{Validate, ValidateDoesNotContain, ValidationError, ValidationErrors};

use crate::database::auth_token::AuthToken;
use crate::database::database::DBPool;
use crate::database::schema::{auth_tokens::dsl::*, inet6_aton, last_insert_id, UserConfirmAction, users::dsl::*, UserStatus};
use crate::database::user::User;
use crate::mailing::mailer::send_email;
use crate::utils::auth::DeviceInfo;
use crate::utils::errors_catcher::{ErrorResponder, ErrorResponse, ErrorType};
use crate::utils::utils::{random_code, random_token};
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
    pub(crate) auth_token: String,
}

#[post("/auth/signup", data = "<data>")]
pub fn auth_signup(data: Json<SignupData>, db: &rocket::State<DBPool>, device_info: DeviceInfo) -> Result<Json<SignupResponse>, ErrorResponder> {
    validate_input(&data)?;
    let conn = &mut db.get().unwrap();

    let conf_code = random_code(4) as u16;
    let conf_token = random_token(16);
    let result = insert_into(users)
        .values((
            name.eq::<String>(data.name.clone()),
            email.eq(data.email.clone()),
            password_hash.eq(bcrypt::hash(data.password.clone()).unwrap()),
            confirm_code.eq(conf_code),
            confirm_token.eq(conf_token.clone()),
            // confirm_action = 'signup', status = 'unconfirmed'
        ))
        .execute(conn).map_err(|e| {
        if let diesel::result::Error::DatabaseError(kind, _) = e {
            if let DatabaseErrorKind::UniqueViolation = kind {
                return ErrorType::EmailAlreadyExists.to_responder();
            }
        }
        ErrorType::DatabaseError("Failed to insert user".to_string(), e).to_responder()
    })?;
    if result == 0 {
        return ErrorType::InvalidInput("Failed to insert user.".to_string()).to_err();
    }
    let uid = select(last_insert_id()).get_result::<u64>(conn).map_err(|e| {
        ErrorType::DatabaseError("Failed to get last insert id".to_string(), e).to_responder()
    })? as u32;

    let auth_token = AuthToken::insert_token_for_user(conn, uid, device_info)?;

    let subject = "Sign up confirmatfion".to_string();
    let body = format!("Hello, the sign up confirmation code is {}, and the url is https://xxx.com/signup/confirm?id={}&token={}",
                       conf_code, uid, hex::encode(conf_token));

    send_email((data.name.clone(), data.email.clone()), subject, body.clone(), format!("<h1>{}</h1>", body));

    Ok(Json(SignupResponse {
        user_id: uid,
        auth_token: hex::encode(auth_token),
    }))
}
