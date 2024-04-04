use diesel::sql_types::{Binary, Nullable, VarChar, SqlType};
use diesel::query_builder::QueryId;
use diesel::{table, joinable, allow_tables_to_appear_in_same_query, Queryable};
use diesel::expression::functions::sql_function;
use serde::Serialize;

sql_function! { fn last_insert_id() -> Unsigned<Bigint> }
sql_function! { fn inet6_ntoa(ip: Nullable<Binary>) -> Nullable<VarChar> }
sql_function! { fn inet6_aton(ip: Nullable<VarChar>) -> Nullable<Varbinary> }
sql_function! { fn utc_timestamp() -> Datetime }

#[derive(Debug, PartialEq, diesel_derive_enum::DbEnum)]
pub enum UserConfirmAction {
    Signup,
    Signin,
    DeleteAccount,
}
#[derive(Debug, PartialEq, Serialize, diesel_derive_enum::DbEnum)]
pub enum UserStatus {
    Unconfirmed,
    Normal,
    Banned,
    Admin,
}
table! {
    use diesel::sql_types::*;
    use super::{UserConfirmActionMapping, UserStatusMapping};
    users (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        email -> Varchar,
        // 60 character
        password_hash -> Char,
        creation_date -> Datetime,
        confirm_date -> Datetime,
        confirm_action -> UserConfirmActionMapping,
        // 16 byte
        confirm_token -> Nullable<Binary>,
        confirm_code -> Nullable<Unsigned<Smallint>>,
        confirm_code_trials -> Unsigned<Tinyint>,
        status -> UserStatusMapping,
        storage_count_mo -> Unsigned<Integer>,
    }
}

table! {
    auth_tokens (user_id, token) {
        user_id -> Unsigned<Integer>,
        token -> Binary,
        creation_date -> Datetime,
        last_use_date -> Datetime,
        device_string -> Nullable<Varchar>,
        ip_address -> Nullable<Varbinary>,
    }
}
joinable!(auth_tokens -> users (user_id));
allow_tables_to_appear_in_same_query!(auth_tokens, users);

table! {
    shares_auto_accept (user_id_acceptor, user_id_sharer) {
        user_id_acceptor -> Unsigned<Integer>,
        user_id_sharer -> Unsigned<Integer>,
    }
}
joinable!(shares_auto_accept -> users (user_id_acceptor));
// joinable!(shares_auto_accept -> users (user_id_sharer));
allow_tables_to_appear_in_same_query!(shares_auto_accept, users);

table! {
    tag_groups (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        name -> Varchar,
        is_multiple -> Bool,
        default_tag_id -> Nullable<Unsigned<Integer>>,
    }
}
joinable!(tag_groups -> users (user_id));
allow_tables_to_appear_in_same_query!(tag_groups, users);

table! {
    tags (id) {
        id -> Unsigned<Integer>,
        tag_group_id -> Unsigned<Integer>,
        name -> Varchar,
        color -> Binary,
    }
}
joinable!(tags -> tag_groups (tag_group_id));
allow_tables_to_appear_in_same_query!(tags, tag_groups);

#[derive(Debug, PartialEq, diesel_derive_enum::DbEnum)]
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
    use super::PictureOrientationMapping;
    pictures (id) {
        id -> Unsigned<BigInt>,
        user_id -> Unsigned<Integer>,
        creation_date -> Datetime,
        edition_date -> Datetime,
        latitude -> Nullable<Decimal>,
        longitude -> Nullable<Decimal>,
        altitude -> Nullable<SmallInt>,
        orientation -> PictureOrientationMapping,
        width -> Unsigned<SmallInt>,
        height -> Unsigned<SmallInt>,
        camera_brand -> Nullable<Varchar>,
        camera_model -> Nullable<Varchar>,
        focal_length -> Nullable<Decimal>,
        exposure_time_num -> Nullable<Unsigned<Integer>>,
        exposure_time_den -> Nullable<Unsigned<Integer>>,
        iso_speed -> Nullable<Unsigned<Integer>>,
        f_number -> Nullable<Decimal>,
    }
}
joinable!(pictures -> users (user_id));
allow_tables_to_appear_in_same_query!(pictures, users);

table! {
    pictures_tags (picture_id, tag_id) {
        picture_id -> Unsigned<BigInt>,
        tag_id -> Unsigned<Integer>,
    }
}
joinable!(pictures_tags -> pictures (picture_id));
joinable!(pictures_tags -> tags (tag_id));
allow_tables_to_appear_in_same_query!(pictures_tags, pictures);
allow_tables_to_appear_in_same_query!(pictures_tags, tags);

table! {
    groups (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        name -> Varchar,
        strategy -> Blob,
    }
}
joinable!(groups -> users (user_id));
allow_tables_to_appear_in_same_query!(groups, users);

table! {
    subgroups (id) {
        id -> Unsigned<Integer>,
        group_id -> Unsigned<Integer>,
        name -> Varchar,
    }
}
joinable!(subgroups -> groups (group_id));
allow_tables_to_appear_in_same_query!(subgroups, groups);

table! {
    subgroups_pictures (subgroup_id, picture_id) {
        subgroup_id -> Unsigned<Integer>,
        picture_id -> Unsigned<BigInt>,
    }
}
joinable!(subgroups_pictures -> subgroups (subgroup_id));
joinable!(subgroups_pictures -> pictures (picture_id));
allow_tables_to_appear_in_same_query!(subgroups_pictures, subgroups);
allow_tables_to_appear_in_same_query!(subgroups_pictures, pictures);

#[derive(Debug, PartialEq, diesel_derive_enum::DbEnum)]
pub enum SharedSubgroupType {
    Unconfirmed,
    Sync,
    Preserve,
}
table! {
    use diesel::sql_types::*;
    use super::SharedSubgroupTypeMapping;
    shared_subgroups (user_id, subgroup_id) {
        user_id -> Unsigned<Integer>,
        subgroup_id -> Unsigned<Integer>,
        #[sql_name="type"]
        pic_type -> SharedSubgroupTypeMapping,
    }
}
joinable!(shared_subgroups -> subgroups (subgroup_id));
joinable!(shared_subgroups -> users (user_id));
allow_tables_to_appear_in_same_query!(shared_subgroups, subgroups);
allow_tables_to_appear_in_same_query!(shared_subgroups, users);

table! {
    hierarchies (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        name -> Varchar,
    }
}
joinable!(hierarchies -> users (user_id));
allow_tables_to_appear_in_same_query!(hierarchies, users);

table! {
    hierarchies_groups (hierarchy_id, group_id) {
        hierarchy_id -> Unsigned<Integer>,
        group_id -> Unsigned<Integer>,
        parent_subgroup_id -> Unsigned<Integer>,
    }
}
joinable!(hierarchies_groups -> hierarchies (hierarchy_id));
joinable!(hierarchies_groups -> groups (group_id));
joinable!(hierarchies_groups -> subgroups (parent_subgroup_id));
allow_tables_to_appear_in_same_query!(hierarchies_groups, hierarchies);
allow_tables_to_appear_in_same_query!(hierarchies_groups, groups);
allow_tables_to_appear_in_same_query!(hierarchies_groups, subgroups);
