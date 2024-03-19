use chrono::{NaiveDateTime, TimeDelta, Utc};
use diesel::{Connection, Identifiable, insert_into, Insertable, Queryable, RunQueryDsl, Selectable, update};
use diesel::ExpressionMethods;

use crate::database::database::DBConn;
use crate::database::schema::*;
use crate::database::schema::auth_tokens;
use crate::utils::auth::DeviceInfo;
use crate::utils::errors_catcher::{ErrorResponder, ErrorType};
use crate::utils::utils::random_token;

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(primary_key(user_id, token))]
#[diesel(belongs_to(User))]
#[diesel(table_name = auth_tokens)]
pub struct AuthToken {
    pub user_id: u32,
    pub token: Vec<u8>,
    pub creation_date: NaiveDateTime,
    pub last_use_date: NaiveDateTime,
    pub device_string: Option<String>,
    pub ip_address: Option<Vec<u8>>,
}

impl AuthToken {
    pub(crate) fn insert_token_for_user(conn: &mut DBConn, user_id: u32, device_info: DeviceInfo) -> Result<Vec<u8>, ErrorResponder> {
        let auth_token = random_token(32);
        insert_into(auth_tokens::table)
            .values((
                auth_tokens::dsl::user_id.eq::<u32>(user_id),
                auth_tokens::dsl::token.eq(auth_token.clone()),
                auth_tokens::dsl::device_string.eq(device_info.device_string),
                auth_tokens::dsl::ip_address.eq(inet6_aton(device_info.ip_address))
            ))
            .execute(conn).map_err(|e| {
            ErrorType::DatabaseError("Failed to insert auth token".to_string(), e).to_responder()
        })?;
        Ok(auth_token)
    }
    pub fn update_last_use_date(&self, conn: &mut DBConn) -> Result<(), ErrorResponder> {
        // Working in UTC time.
        let current_naive = Utc::now().naive_utc();
        if current_naive - self.last_use_date > TimeDelta::try_minutes(10).unwrap() {
            println!("Updating last_use_date");
            update(auth_tokens::table)
                .filter(auth_tokens::dsl::user_id.eq(self.user_id))
                .filter(auth_tokens::dsl::token.eq(self.token.clone()))
                .set((
                    auth_tokens::dsl::last_use_date.eq(utc_timestamp()),
                ))
                .execute(conn).map_err(|e| {
                ErrorType::DatabaseError("Failed to update auth token use date".to_string(), e).to_responder()
            })?;
        }
        Ok(())
    }
}
