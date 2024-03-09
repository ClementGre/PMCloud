use diesel::{Associations, Identifiable, Queryable, Selectable};
use crate::database::schema::*;
use crate::database::{user::User, picture::Picture};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = groups)]
pub struct Group {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub strategy: Vec<u8>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = subgroups)]
pub struct Subgroup {
    pub id: i32,
    pub group_id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(subgroup_id, picture_id))]
#[diesel(belongs_to(Subgroup))]
#[diesel(belongs_to(Picture))]
#[diesel(table_name = subgroups_pictures)]
pub struct SubgroupPicture {
    pub subgroup_id: i32,
    pub picture_id: i64,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(user_id, subgroup_id))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Subgroup))]
pub struct SharedSubgroup {
    pub user_id: i32,
    pub subgroup_id: i32,
    pub pic_type: SharedSubgroupType,
}

impl Group {}

impl Subgroup {}

impl SubgroupPicture {}

impl SharedSubgroup {}
