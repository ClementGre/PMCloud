#[macro_use] extern crate rocket;

use diesel::migration::MigrationSource;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;

use crate::api::auth::signup::{auth_signup, SignupData, SignupResponse};
use crate::database::database::{DBPool, get_connection, get_connection_pool};
use crate::utils::errors_catcher::{bad_request, ErrorResponder, internal_error, not_found, unauthorized, unprocessable_entity};

mod api {
    pub mod admin {
        pub mod admin;
    }
    pub mod auth {
        pub mod signup;
        pub mod signin;
        pub mod confirm;
    }
}
mod database {
    pub mod database;
    pub mod schema;
    pub mod user;
    pub mod tags;
    pub mod picture;
    pub mod group;
    pub mod hierarchy;
}
mod ftp_server {
    pub mod ftp;
    pub mod ftp_auth;
    pub mod ftp_backend;
}
mod grouping {
    pub mod grouping_strategy;
}
mod utils {
    pub mod utils;
    pub mod errors_catcher;
    pub mod validation;
    pub mod auth;
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[launch]
fn rocket() -> _ {

    // migrate database
    let mut conn = get_connection();
    let res = conn.run_pending_migrations(MIGRATIONS).unwrap();
    println!("Migrations result: {:?}", res);

    rocket::build()
        .manage(get_connection_pool())
        .mount("/", routes![auth_signup])
        .register("/", catchers![bad_request, unauthorized, not_found, unprocessable_entity, internal_error])
}

#[derive(Debug, Deserialize)]
struct Test {
    user_id: u32,
    name: String,
    age: u32,
}




