use diesel::{Associations, Identifiable, Queryable, Selectable};
use crate::database::schema::*;
use crate::database::{user::User, picture::Picture};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(table_name = tag_groups)]
pub struct TagGroup {
    pub id: u32,
    pub user_id: u32,
    pub name: String,
    pub is_multiple: bool,
    pub default_tag_id: Option<u32>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(TagGroup))]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: u32,
    pub tag_group_id: u32,
    pub name: String,
    pub color: Vec<u8>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(picture_id, tag_id))]
#[diesel(belongs_to(Picture))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = pictures_tags)]
pub struct PictureTag {
    pub picture_id: u64,
    pub tag_id: u32,
}

impl TagGroup {}

impl Tag {}

impl PictureTag {}

