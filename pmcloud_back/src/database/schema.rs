use diesel::sql_types::SqlType;
use diesel::query_builder::QueryId;
use diesel::{table, joinable, allow_tables_to_appear_in_same_query};

#[derive(SqlType, QueryId, Debug, PartialEq)]
pub enum UserConfirmAction {
    Signup,
    Signin,
    DeleteAccount,
}
#[derive(SqlType, QueryId, Debug, PartialEq)]
pub enum UserStatus {
    Unconfirmed,
    Normal,
    Banned,
    Admin,
}
table! {
    use diesel::sql_types::*;
    use super::{UserConfirmAction, UserStatus};
    users (id) {
        id -> Unsigned<Int4>,
        name -> Varchar,
        email -> Varchar,
        /// 60 character
        password_hash -> Char,
        confirm_date -> Datetime,
        confirm_action -> UserConfirmAction,
        /// 16 byte
        confirm_token -> Binary,
        confirm_code -> Unsigned<Smallint>,
        confirm_code_trials -> Unsigned<Tinyint>,
        auth_token -> Binary,
        status -> UserStatus,
        storage_count_mo -> Unsigned<Int4>,
    }
}

table! {
    shares_auto_accept (user_id_acceptor, user_id_sharer) {
        user_id_acceptor -> Unsigned<Int4>,
        user_id_sharer -> Unsigned<Int4>,
    }
}
joinable!(shares_auto_accept -> users (user_id_acceptor));
// joinable!(shares_auto_accept -> users (user_id_sharer));
allow_tables_to_appear_in_same_query!(shares_auto_accept, users);

table! {
    tag_groups (id) {
        id -> Unsigned<Int4>,
        user_id -> Unsigned<Int4>,
        name -> Varchar,
        is_multiple -> Bool,
        default_tag_id -> Nullable<Unsigned<Int4>>,
    }
}
joinable!(tag_groups -> users (user_id));
allow_tables_to_appear_in_same_query!(tag_groups, users);

table! {
    tags (id) {
        id -> Unsigned<Int4>,
        tag_group_id -> Unsigned<Int4>,
        name -> Varchar,
        color -> Binary,
    }
}
joinable!(tags -> tag_groups (tag_group_id));
allow_tables_to_appear_in_same_query!(tags, tag_groups);

#[derive(SqlType, QueryId, Debug, PartialEq)]
pub enum PictureOrientation {
    Unspecified,
    Normal,
    HorizontalFlip,
    Rotate180,
    VerticalFlip,
    Rotate90HorizontalFlip,
    Rotate90,
    Rotate90VerticalFlip,
    Rotate270,
}

table! {
    use diesel::sql_types::*;
    use super::PictureOrientation;
    pictures (id) {
        id -> Unsigned<Int8>,
        user_id -> Unsigned<Int4>,
        creation_date -> Datetime,
        edition_date -> Datetime,
        latitude -> Nullable<Decimal>,
        longitude -> Nullable<Decimal>,
        altitude -> Nullable<Int2>,
        orientation -> PictureOrientation,
        width -> Unsigned<Int2>,
        height -> Unsigned<Int2>,
        camera_brand -> Nullable<Varchar>,
        camera_model -> Nullable<Varchar>,
        focal_length -> Nullable<Decimal>,
        exposure_time_num -> Nullable<Unsigned<Int4>>,
        exposure_time_den -> Nullable<Unsigned<Int4>>,
        iso_speed -> Nullable<Unsigned<Int4>>,
        f_number -> Nullable<Decimal>,
    }
}
joinable!(pictures -> users (user_id));
allow_tables_to_appear_in_same_query!(pictures, users);

table! {
    pictures_tags (picture_id, tag_id) {
        picture_id -> Unsigned<Int8>,
        tag_id -> Unsigned<Int4>,
    }
}
joinable!(pictures_tags -> pictures (picture_id));
joinable!(pictures_tags -> tags (tag_id));
allow_tables_to_appear_in_same_query!(pictures_tags, pictures);
allow_tables_to_appear_in_same_query!(pictures_tags, tags);

table! {
    groups (id) {
        id -> Unsigned<Int4>,
        user_id -> Unsigned<Int4>,
        name -> Varchar,
        strategy -> Blob,
    }
}
joinable!(groups -> users (user_id));
allow_tables_to_appear_in_same_query!(groups, users);

table! {
    subgroups (id) {
        id -> Unsigned<Int4>,
        group_id -> Unsigned<Int4>,
        name -> Varchar,
    }
}
joinable!(subgroups -> groups (group_id));
allow_tables_to_appear_in_same_query!(subgroups, groups);

table! {
    subgroups_pictures (subgroup_id, picture_id) {
        subgroup_id -> Unsigned<Int4>,
        picture_id -> Unsigned<Int8>,
    }
}
joinable!(subgroups_pictures -> subgroups (subgroup_id));
joinable!(subgroups_pictures -> pictures (picture_id));
allow_tables_to_appear_in_same_query!(subgroups_pictures, subgroups);
allow_tables_to_appear_in_same_query!(subgroups_pictures, pictures);

#[derive(SqlType, QueryId, Debug, PartialEq)]
pub enum SharedSubgroupType {
    Unconfirmed,
    Sync,
    Preserve,
}
table! {
    use diesel::sql_types::*;
    use super::SharedSubgroupType;
    shared_subgroups (user_id, subgroup_id) {
        user_id -> Unsigned<Int4>,
        subgroup_id -> Unsigned<Int4>,
        #[sql_name="type"]
        pic_type -> SharedSubgroupType,
    }
}
joinable!(shared_subgroups -> subgroups (subgroup_id));
joinable!(shared_subgroups -> users (user_id));
allow_tables_to_appear_in_same_query!(shared_subgroups, subgroups);
allow_tables_to_appear_in_same_query!(shared_subgroups, users);

table! {
    hierarchies (id) {
        id -> Unsigned<Int4>,
        user_id -> Unsigned<Int4>,
        name -> Varchar,
    }
}
joinable!(hierarchies -> users (user_id));
allow_tables_to_appear_in_same_query!(hierarchies, users);

table! {
    hierarchies_groups (hierarchy_id, group_id) {
        hierarchy_id -> Unsigned<Int4>,
        group_id -> Unsigned<Int4>,
        parent_subgroup_id -> Unsigned<Int4>,
    }
}
joinable!(hierarchies_groups -> hierarchies (hierarchy_id));
joinable!(hierarchies_groups -> groups (group_id));
joinable!(hierarchies_groups -> subgroups (parent_subgroup_id));
allow_tables_to_appear_in_same_query!(hierarchies_groups, hierarchies);
allow_tables_to_appear_in_same_query!(hierarchies_groups, groups);
allow_tables_to_appear_in_same_query!(hierarchies_groups, subgroups);
