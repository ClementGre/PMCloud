use diesel::prelude::*;
use diesel::prelude::*;
use rocket::form::validate::Contains;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::database::database::DBPool;
use crate::database::schema::*;
use crate::database::user::{AuthToken, User};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // get user_id and auth_token from request headers
        let user_id = request.headers().get_one("X-User-Id").map(|s| s.parse::<u32>().ok()).flatten();
        let auth_token = request.headers().get_one("X-Auth-Token").map(|s| hex::decode(s).ok()).flatten();
        let session_id = request.headers().get_one("X-Session-Id").map(|s| s.parse::<u16>().ok()).flatten();

        if user_id.is_none() || auth_token.is_none() || session_id.is_none() {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let db: &DBPool = request.rocket().state::<DBPool>().unwrap();
        let conn = &mut db.get().unwrap();

        let result = users::table.left_join(auth_tokens::table)
            .filter(users::dsl::id.eq(user_id.unwrap()))
            .select((User::as_select(), Option::<AuthToken>::as_select()))
            .first::<(User, Option<AuthToken>)>(conn);

        if result.is_err() {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        if let Some((user, Some(auth))) = result.ok() {
            if auth.token == auth_token.unwrap() && auth.last_session_id == session_id.unwrap() {
                return Outcome::Success(user);
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub(crate) user_agent: Option<String>,
    pub(crate) ip_address: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DeviceInfo {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_agent = request.headers().get_one("user-agent").map(|s| s.to_string());
        let mut ip_address = request.remote().map(|s| s.to_string()).or(request.headers().get_one("X-Forwarded-For").map(|s| s.to_string()));

        // removing port from ip address even if it is an ipv6
        if let Some(ip) = ip_address.clone() {
            if ip.contains(':'){
                if ip.chars().filter(|c| *c == 'z').count() > 1 {
                    if ip.starts_with('[') && ip.contains("]") {
                        ip_address = Some(ip[1..ip.find("]").unwrap()].to_string());
                    }
                }else{
                    ip_address = Some(ip[0..ip.find(":").unwrap()].to_string());
                }
            }
        }

        Outcome::Success(DeviceInfo {
            user_agent,
            ip_address,
        })
    }
}
