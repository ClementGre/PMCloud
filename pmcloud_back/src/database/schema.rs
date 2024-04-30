use diesel::{allow_tables_to_appear_in_same_query, joinable, Queryable, table};
use diesel::expression::functions::sql_function;
use diesel::query_builder::QueryId;
use diesel::sql_types::{Binary, Nullable, SqlType, VarChar};
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
        multiple -> Bool,
        default_tag_id -> Nullable<Unsigned<Integer>>,
        required -> Bool
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
        is_default -> Bool,
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
        owner_id -> Unsigned<Integer>,
        author_id -> Unsigned<Integer>,
        deleted_date -> Nullable<Datetime>,
        copied -> Bool,
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
joinable!(pictures -> users (owner_id));
//joinable!(pictures -> users (author_id));
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
    arrangements (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        name -> Varchar,
        match_conversion -> Bool,
        strategy -> Blob,
    }
}
joinable!(arrangements -> users (user_id));
allow_tables_to_appear_in_same_query!(arrangements, users);

table! {
    groups (id) {
        id -> Unsigned<Integer>,
        arrangement_id -> Unsigned<Integer>,
        name -> Varchar,
        share_link -> Nullable<Unsigned<BigInt>>,
    }
}
joinable!(groups -> arrangements (arrangement_id));
allow_tables_to_appear_in_same_query!(groups, arrangements);

table! {
    groups_pictures (group_id, picture_id) {
        group_id -> Unsigned<Integer>,
        picture_id -> Unsigned<BigInt>,
    }
}
joinable!(groups_pictures -> groups (group_id));
joinable!(groups_pictures -> pictures (picture_id));
allow_tables_to_appear_in_same_query!(groups_pictures, groups);
allow_tables_to_appear_in_same_query!(groups_pictures, pictures);

table! {
    link_share_groups (token) {
        token -> Binary,
        group_id -> Unsigned<Integer>,
        permissions -> Unsigned<TinyInt>,
    }
}
joinable!(link_share_groups -> groups (group_id));
allow_tables_to_appear_in_same_query!(link_share_groups, groups);

table! {
    use diesel::sql_types::*;
    shared_groups (user_id, group_id) {
        user_id -> Unsigned<Integer>,
        group_id -> Unsigned<Integer>,
        permissions -> Unsigned<TinyInt>,
        match_conversion_group_id -> Nullable<Unsigned<Integer>>,
        copied -> Bool,
        confirmed -> Bool,
    }
}
joinable!(shared_groups -> groups (group_id));
joinable!(shared_groups -> users (user_id));
//joinable!(shared_groups -> groups (match_conversion_group_id));
allow_tables_to_appear_in_same_query!(shared_groups, groups);
allow_tables_to_appear_in_same_query!(shared_groups, users);

table! {
    hierarchies (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        name -> Varchar,
        match_conversion -> Bool,
    }
}
joinable!(hierarchies -> users (user_id));
allow_tables_to_appear_in_same_query!(hierarchies, users);

table! {
    hierarchies_arrangements(hierarchy_id, arrangement_id) {
        hierarchy_id -> Unsigned<Integer>,
        arrangement_id -> Unsigned<Integer>,
        parent_group_id -> Unsigned<Integer>,
    }
}
joinable!(hierarchies_arrangements -> hierarchies (hierarchy_id));
joinable!(hierarchies_arrangements -> arrangements (arrangement_id));
joinable!(hierarchies_arrangements -> groups (parent_group_id));
allow_tables_to_appear_in_same_query!(hierarchies_arrangements, hierarchies);
allow_tables_to_appear_in_same_query!(hierarchies_arrangements, arrangements);
allow_tables_to_appear_in_same_query!(hierarchies_arrangements, groups);
