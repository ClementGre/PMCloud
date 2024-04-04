use std::fmt::Display;
use std::ops::AddAssign;

use diesel::prelude::*;
use diesel::prelude::*;
use rocket::form::validate::Contains;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use user_agent_parser::{Device, Engine, OS};

use crate::database::auth_token::AuthToken;
use crate::database::database::DBPool;
use crate::database::schema::*;
use crate::database::user::User;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // get user_id and auth_token from request headers
        let user_id = request.headers().get_one("X-User-Id").map(|s| s.parse::<u32>().ok()).flatten();
        let auth_token = request.headers().get_one("X-Auth-Token").map(|s| hex::decode(s).ok()).flatten();
        if user_id.is_none() || auth_token.is_none() {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let db: &DBPool = request.rocket().state::<DBPool>().unwrap();
        let conn = &mut db.get().unwrap();

        let result = users::table.left_join(auth_tokens::table)
            .filter(users::dsl::id.eq(user_id.unwrap()))
            .filter(auth_tokens::dsl::token.eq(auth_token.unwrap()))
            .select((User::as_select(), Option::<AuthToken>::as_select()))
            .first::<(User, Option<AuthToken>)>(conn);

        if let Some((user, Some(auth))) = result.ok() {
            let result = auth.update_last_use_date(conn);
            if result.is_err() {
                // TODO: log the error but keep the response as successful
            }
            return Outcome::Success(user);
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}


#[derive(Debug)]
pub struct DeviceInfo {
    pub(crate) device_string: String,
    pub(crate) ip_address: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DeviceInfo {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut ip_address = request.remote().map(|s| s.to_string()).or(request.headers().get_one("X-Forwarded-For").map(|s| s.to_string()));

        let device = Device::from_request(request).await.unwrap();
        let os = OS::from_request(request).await.unwrap();
        let engine = Engine::from_request(request).await.unwrap();

        let device_string = device_str(device, os, engine);

        // removing port from ip address even if it is an ipv6
        if let Some(ip) = ip_address.clone() {
            if ip.contains(':') {
                if ip.chars().filter(|c| *c == 'z').count() > 1 {
                    if ip.starts_with('[') && ip.contains("]") {
                        ip_address = Some(ip[1..ip.find("]").unwrap()].to_string());
                    }
                } else {
                    ip_address = Some(ip[0..ip.find(":").unwrap()].to_string());
                }
            }
        }

        Outcome::Success(DeviceInfo {
            device_string,
            ip_address,
        })
    }
}

fn device_str(device: Device, os: OS, engine: Engine) -> String {
    let mut device_str = String::new();

    if let Some(brand) = device.brand {
        device_str = format!("{} ", brand);
    }
    if let Some(name) = device.name {
        device_str.add_assign(format!("{} ", name).as_str());
    } else if let Some(model) = device.model {
        device_str.add_assign(format!("{} ", model).as_str());
    }

    if let Some(name) = os.name {
        device_str.add_assign(format!("({}", name).as_str());
        if let Some(major) = os.major {
            device_str.add_assign(format!(" {}", major).as_str());
            if let Some(minor) = os.minor {
                device_str.add_assign(format!(".{}", minor).as_str());
                if let Some(patch) = os.patch {
                    device_str.add_assign(format!(".{}", patch).as_str());
                    if let Some(patch_minor) = os.patch_minor {
                        device_str.add_assign(format!(".{}", patch_minor).as_str());
                    }
                }
            }
        }
        device_str.add_assign(") ");
    }

    if let Some(name) = engine.name {
        device_str.add_assign(format!("{}", name).as_str());
        if let Some(major) = engine.major {
            device_str.add_assign(format!(" {}", major).as_str());
            if let Some(minor) = engine.minor {
                device_str.add_assign(format!(".{}", minor).as_str());
                if let Some(patch) = engine.patch {
                    device_str.add_assign(format!(".{}", patch).as_str());
                }
            }
        }
    }

    if device_str.is_empty() {
        device_str = "Unknown".to_string();
    }
    device_str
}
