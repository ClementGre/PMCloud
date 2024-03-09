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
}

#[macro_use] extern crate rocket;

use diesel::migration::MigrationSource;
use crate::database::database::{get_connection, get_connection_pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::prelude::*;
use crate::api::auth::signup::auth_signup;


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
}




