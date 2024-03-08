# CREATE TABLE users
# (
#     uid INT AUTO_INCREMENT,
#     name VARCHAR(32) NOT NULL,
#     email VARCHAR(256) NOT NULL UNIQUE,
#     password_hash CHAR(64) NOT NULL,
#     confirm_date DATETIME NOT NULL DEFAULT 1970-01-01 00:00:00,
#     confirm_action ENUM(signup, signin, delete_account) NOT NULL DEFAULT signup,
#     confirm_token BINARY(16) NOT NULL,
#     confirm_code SMALLINT UNSIGNED NOT NULL,
#     confirm_code_trials TINYINT UNSIGNED NOT NULL DEFAULT 0,
#     auth_token BINARY(16) NOT NULL,
#     status ENUM(unconfirmed, normal, banned, admin) NOT NULL DEFAULT unconfirmed,
#     storage_count_mo INT UNSIGNED NOT NULL DEFAULT 0,
#     CONSTRAINT PK_users PRIMARY KEY (uid)
# );

# CREATE TABLE shares_auto_accept
# (
#     CONSTRAINT PK_shares_auto_accept PRIMARY KEY (user_id_acceptor, user_id_sharer),
#     user_id_acceptor INT,
#     user_id_sharer INT,
#     FOREIGN KEY (user_id_acceptor) REFERENCES users(uid),
#     FOREIGN KEY (user_id_sharer) REFERENCES users(uid)
# );

# CREATE TABLE tag_groups
# (
#     CONSTRAINT PK_tag_groups PRIMARY KEY (uid),
#     uid INT AUTO_INCREMENT,
#     user_uid INT NOT NULL,
#     name VARCHAR(32) NOT NULL,
#     is_multiple BOOLEAN NOT NULL DEFAULT FALSE,
#     default_tag_uid INT UNSIGNED,
#     FOREIGN KEY (user_uid) REFERENCES users(uid)
# );
#
# CREATE TABLE tags
# (
#     CONSTRAINT PK_tags PRIMARY KEY (uid),
#     uid INT AUTO_INCREMENT,
#     group_uid INT NOT NULL,
#     name VARCHAR(32) NOT NULL,
#     color BINARY(3) NOT NULL DEFAULT 0x000000,
#     FOREIGN KEY (group_uid) REFERENCES tag_groups(uid)
# );

CREATE TABLE pictures
(
    CONSTRAINT PK_photos PRIMARY KEY (uid),
    uid BIGINT AUTO_INCREMENT,
    user_uid INT NOT NULL,
    creation_date DATETIME NOT NULL,
    edition_date DATETIME NOT NULL,
    latitude DECIMAL(8,6),
    longitude DECIMAL(9,6),
    altitude SMALLINT,
    orientation ENUM('Unspecified', 'Normal', 'HorizontalFlip', 'Rotate180', 'VerticalFlip', 'Rotate90HorizontalFlip', 'Rotate90', 'Rotate90VerticalFlip', 'Rotate270') NOT NULL DEFAULT 'Unspecified',
    width SMALLINT UNSIGNED NOT NULL,
    height SMALLINT UNSIGNED NOT NULL,
    camera_brand VARCHAR(32),
    camera_model VARCHAR(32),
    focal_length DECIMAL(6,2),
    exposure_time_num INT UNSIGNED,
    exposure_time_den INT UNSIGNED,
    iso_speed INT UNSIGNED,
    f_number DECIMAL(4,1),
    FOREIGN KEY (user_uid) REFERENCES users(uid)
);

CREATE TABLE pictures_tags
(
    CONSTRAINT PK_pictures_tags PRIMARY KEY (picture_uid, tag_uid),
    picture_uid BIGINT,
    tag_uid INT,
    FOREIGN KEY (picture_uid) REFERENCES pictures(uid),
    FOREIGN KEY (tag_uid) REFERENCES tags(uid)
);

CREATE TABLE `groups`
(
    CONSTRAINT PK_groups PRIMARY KEY (uid),
    uid INT AUTO_INCREMENT,
    name VARCHAR(32) NOT NULL,
    strategy BLOB NOT NULL,
    owner_uid INT NOT NULL,
    FOREIGN KEY (owner_uid) REFERENCES users(uid)
);

CREATE TABLE subgroups
(
    CONSTRAINT PK_subgroups PRIMARY KEY (uid),
    uid INT AUTO_INCREMENT,
    group_uid INT NOT NULL,
    name VARCHAR(32) NOT NULL,
    FOREIGN KEY (group_uid) REFERENCES `groups`(uid)
);

CREATE TABLE subgroups_pictures
(
    CONSTRAINT PK_subgroups_pictures PRIMARY KEY (subgroup_uid, picture_uid),
    subgroup_uid INT,
    picture_uid BIGINT,
    FOREIGN KEY (subgroup_uid) REFERENCES subgroups(uid),
    FOREIGN KEY (picture_uid) REFERENCES pictures(uid)
);

CREATE TABLE shared_subgroups
(
    CONSTRAINT PK_shared_subgroups PRIMARY KEY (user_uid, subgroup_uid),
    user_uid INT,
    subgroup_uid INT,
    type ENUM('unconfirmed', 'sync', 'preserve') DEFAULT 'unconfirmed' NOT NULL,
    FOREIGN KEY (user_uid) REFERENCES users(uid),
    FOREIGN KEY (subgroup_uid) REFERENCES subgroups(uid)
);

CREATE TABLE hierarchy
(
    CONSTRAINT PK_hierarchy PRIMARY KEY (uid),
    uid INT AUTO_INCREMENT,
    name VARCHAR(32) NOT NULL
);

CREATE TABLE hierarchy_groups
(
    CONSTRAINT PK_hierarchy_groups PRIMARY KEY (hierarchy_uid, group_uid),
    hierarchy_uid INT,
    group_uid INT,
    parent_subgroup_uid INT NOT NULL ,
    FOREIGN KEY (hierarchy_uid) REFERENCES hierarchy(uid),
    FOREIGN KEY (group_uid) REFERENCES `groups`(uid)
);
