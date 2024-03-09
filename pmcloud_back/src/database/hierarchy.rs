use diesel::{Associations, Identifiable, Queryable, Selectable};
use crate::database::schema::*;
use crate::database::{user::User, group::{Group, Subgroup}};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = hierarchies)]
pub struct Hierarchy {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(hierarchy_id, group_id))]
#[diesel(belongs_to(Hierarchy))]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Subgroup, foreign_key = parent_subgroup_id))]
#[diesel(table_name = hierarchies_groups)]
pub struct HierarchyGroups {
    pub hierarchy_id: i32,
    pub group_id: i32,
    pub parent_subgroup_id: i32,
}

impl Hierarchy {}

impl HierarchyGroups {}
