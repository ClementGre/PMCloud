use diesel::{Associations, Identifiable, Queryable, Selectable};
use crate::database::schema::*;
use crate::database::{user::User, group::{Group, Subgroup}};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(table_name = hierarchies)]
pub struct Hierarchy {
    pub id: u32,
    pub user_id: u32,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(primary_key(hierarchy_id, group_id))]
#[diesel(belongs_to(Hierarchy))]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Subgroup, foreign_key = parent_subgroup_id))]
#[diesel(table_name = hierarchies_groups)]
pub struct HierarchyGroups {
    pub hierarchy_id: u32,
    pub group_id: u32,
    pub parent_subgroup_id: u32,
}

impl Hierarchy {}

impl HierarchyGroups {}
