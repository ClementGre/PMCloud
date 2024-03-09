use diesel::{Associations, Identifiable, Queryable, Selectable};
use time::PrimitiveDateTime;
use crate::database::schema::*;

use crate::database::schema::UserConfirmAction;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub confirm_date: PrimitiveDateTime,
    pub confirm_action: UserConfirmAction,
    pub confirm_token: Vec<u8>,
    pub confirm_code: i16,
    pub confirm_code_trials: i8,
    pub auth_token: Vec<u8>,
    pub status: String,
    pub storage_count_mo: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(user_id_acceptor, user_id_sharer))]
#[diesel(belongs_to(User, foreign_key = user_id_acceptor, foreign_key = user_id_sharer))]
#[diesel(table_name = shares_auto_accept)]
pub struct ShareAutoAccept {
    pub user_id_acceptor: i32,
    pub user_id_sharer: i32,
}

impl User {}

impl ShareAutoAccept {}
