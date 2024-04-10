CREATE TABLE users
(
    id                  INT UNSIGNED AUTO_INCREMENT,
    name                VARCHAR(32)                                       NOT NULL,
    email               VARCHAR(256)                                      NOT NULL UNIQUE,
    password_hash       CHAR(60)                                          NOT NULL,
    creation_date       DATETIME                                          NOT NULL DEFAULT (UTC_TIMESTAMP()),
    confirm_date        DATETIME                                          NOT NULL DEFAULT (UTC_TIMESTAMP()),
    confirm_action      ENUM ('signup', 'signin', 'delete_account')       NOT NULL DEFAULT 'signup',
    confirm_token       BINARY(16)                                                 DEFAULT NULL,
    confirm_code        SMALLINT UNSIGNED                                          DEFAULT NULL,
    confirm_code_trials TINYINT UNSIGNED                                  NOT NULL DEFAULT 0,
    status              ENUM ('unconfirmed', 'normal', 'banned', 'admin') NOT NULL DEFAULT 'unconfirmed',
    storage_count_mo    INT UNSIGNED                                      NOT NULL DEFAULT 0,
    CONSTRAINT PK_users PRIMARY KEY (id)
);

CREATE TABLE auth_tokens
(
    CONSTRAINT PK_auth_tokens PRIMARY KEY (user_id, token),
    user_id       INT UNSIGNED NOT NULL,
    token         BINARY(32)   NOT NULL,
    creation_date DATETIME     NOT NULL DEFAULT (UTC_TIMESTAMP()),
    last_use_date DATETIME     NOT NULL DEFAULT (UTC_TIMESTAMP()),
    device_string VARCHAR(128),
    ip_address    VARBINARY(16),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE shares_auto_accept
(
    CONSTRAINT PK_shares_auto_accept PRIMARY KEY (user_id_acceptor, user_id_sharer),
    user_id_acceptor INT UNSIGNED,
    user_id_sharer   INT UNSIGNED,
    FOREIGN KEY (user_id_acceptor) REFERENCES users (id),
    FOREIGN KEY (user_id_sharer) REFERENCES users (id)
);

CREATE TABLE tag_groups
(
    CONSTRAINT PK_tag_groups PRIMARY KEY (id),
    id             INT UNSIGNED AUTO_INCREMENT,
    user_id        INT UNSIGNED NOT NULL,
    name           VARCHAR(32)  NOT NULL,
    is_multiple    BOOLEAN      NOT NULL DEFAULT FALSE,
    default_tag_id INT UNSIGNED,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE tags
(
    CONSTRAINT PK_tags PRIMARY KEY (id),
    id           INT UNSIGNED AUTO_INCREMENT,
    tag_group_id INT UNSIGNED NOT NULL,
    name         VARCHAR(32)  NOT NULL,
    color        BINARY(3)    NOT NULL DEFAULT 0x000000,
    FOREIGN KEY (tag_group_id) REFERENCES tag_groups (id)
);

CREATE TABLE pictures
(
    CONSTRAINT PK_photos PRIMARY KEY (id),
    id                BIGINT UNSIGNED AUTO_INCREMENT,
    user_id           INT UNSIGNED                                                                                                                                             NOT NULL,
    creation_date     DATETIME                                                                                                                                                 NOT NULL,
    edition_date      DATETIME                                                                                                                                                 NOT NULL,
    latitude          DECIMAL(8, 6),
    longitude         DECIMAL(9, 6),
    altitude          SMALLINT,
    orientation       ENUM ('Unspecified', 'Normal', 'HorizontalFlip', 'Rotate180', 'VerticalFlip', 'Rotate90HorizontalFlip', 'Rotate90', 'Rotate90VerticalFlip', 'Rotate270') NOT NULL DEFAULT 'Unspecified',
    width             SMALLINT UNSIGNED                                                                                                                                        NOT NULL,
    height            SMALLINT UNSIGNED                                                                                                                                        NOT NULL,
    camera_brand      VARCHAR(32),
    camera_model      VARCHAR(32),
    focal_length      DECIMAL(6, 2),
    exposure_time_num INT UNSIGNED,
    exposure_time_den INT UNSIGNED,
    iso_speed         INT UNSIGNED,
    f_number          DECIMAL(4, 1),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE pictures_tags
(
    CONSTRAINT PK_pictures_tags PRIMARY KEY (picture_id, tag_id),
    picture_id BIGINT UNSIGNED,
    tag_id     INT UNSIGNED,
    FOREIGN KEY (picture_id) REFERENCES pictures (id),
    FOREIGN KEY (tag_id) REFERENCES tags (id)
);

CREATE TABLE `groups`
(
    CONSTRAINT PK_groups PRIMARY KEY (id),
    id       INT UNSIGNED AUTO_INCREMENT,
    user_id  INT UNSIGNED NOT NULL,
    name     VARCHAR(32)  NOT NULL,
    strategy BLOB         NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE subgroups
(
    CONSTRAINT PK_subgroups PRIMARY KEY (id),
    id       INT UNSIGNED AUTO_INCREMENT,
    group_id INT UNSIGNED NOT NULL,
    name     VARCHAR(32)  NOT NULL,
    FOREIGN KEY (group_id) REFERENCES `groups` (id)
);

CREATE TABLE subgroups_pictures
(
    CONSTRAINT PK_subgroups_pictures PRIMARY KEY (subgroup_id, picture_id),
    subgroup_id INT UNSIGNED,
    picture_id  BIGINT UNSIGNED,
    FOREIGN KEY (subgroup_id) REFERENCES subgroups (id),
    FOREIGN KEY (picture_id) REFERENCES pictures (id)
);

CREATE TABLE shared_subgroups
(
    CONSTRAINT PK_shared_subgroups PRIMARY KEY (user_id, subgroup_id),
    user_id     INT UNSIGNED,
    subgroup_id INT UNSIGNED,
    type        ENUM ('unconfirmed', 'sync', 'preserve') DEFAULT 'unconfirmed' NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (subgroup_id) REFERENCES subgroups (id)
);

CREATE TABLE hierarchies
(
    CONSTRAINT PK_hierarchy PRIMARY KEY (id),
    id      INT UNSIGNED AUTO_INCREMENT,
    user_id INT UNSIGNED NOT NULL,
    name    VARCHAR(32)  NOT NULL
);

CREATE TABLE hierarchies_groups
(
    CONSTRAINT PK_hierarchy_groups PRIMARY KEY (hierarchy_id, group_id),
    hierarchy_id       INT UNSIGNED,
    group_id           INT UNSIGNED,
    parent_subgroup_id INT UNSIGNED NOT NULL,
    FOREIGN KEY (hierarchy_id) REFERENCES hierarchies (id),
    FOREIGN KEY (group_id) REFERENCES `groups` (id)
);
