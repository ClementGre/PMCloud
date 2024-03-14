use chrono::NaiveDateTime;
use diesel::{Associations, Connection, Identifiable, Insertable, Queryable, Selectable};
use diesel::ExpressionMethods;

use crate::database::schema::*;

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub creation_date: NaiveDateTime,
    pub confirm_date: NaiveDateTime,
    pub confirm_action: UserConfirmAction,
    pub confirm_token: Option<Vec<u8>>,
    pub confirm_code: Option<u16>,
    pub confirm_code_trials: u8,
    pub status: UserStatus,
    pub storage_count_mo: u32,
}

pub struct AuthenticatedUser {}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(user_id_acceptor, user_id_sharer))]
#[diesel(belongs_to(User, foreign_key = user_id_acceptor, foreign_key = user_id_sharer))]
#[diesel(table_name = shares_auto_accept)]
pub struct ShareAutoAccept {
    pub user_id_acceptor: u32,
    pub user_id_sharer: u32,
}

impl User {}

impl ShareAutoAccept {}
