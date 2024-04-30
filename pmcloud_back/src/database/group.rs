use diesel::{Associations, Identifiable, Queryable, Selectable};
use crate::database::schema::*;
use crate::database::{user::User, picture::Picture};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = arrangements)]
pub struct Arrangement {
    pub id: u32,
    pub user_id: u32,
    pub name: String,
    pub match_conversion: bool,
    pub strategy: Vec<u8>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Arrangement))]
#[diesel(table_name = groups)]
pub struct Group {
    pub id: u32,
    pub arrangement_id: u32,
    pub name: String,
    pub share_link: Option<u64>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(group_id, picture_id))]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Picture))]
#[diesel(table_name = groups_pictures)]
pub struct GroupPicture {
    pub group_id: u32,
    pub picture_id: u64,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(token))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = link_share_groups)]
pub struct LinkShareGroups {
    pub token: Vec<u8>,
    pub group_id: u32,
    pub permissions: u8,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(user_id, group_id))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = shared_groups)]
pub struct SharedGroup {
    pub user_id: u32,
    pub group_id: u32,
    pub permissions: Vec<u8>,
    pub match_conversion_group_id: Option<u32>,
    pub copied: bool,
    pub confirmed: bool,
}

impl Arrangement {}

impl Group {}

impl GroupPicture {}

impl SharedGroup {}
