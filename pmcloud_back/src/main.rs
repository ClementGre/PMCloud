#[macro_use]
extern crate rocket;
extern crate tera;

use std::env;
use diesel::migration::MigrationSource;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::http::Method;
use rocket::response::content;
use rocket::response::content::RawHtml;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use serde::Deserialize;
use user_agent_parser::UserAgentParser;

use crate::api::auth::signin::auth_signin;
use crate::api::auth::signup::auth_signup;
use crate::api::auth::status::auth_status;
use crate::database::database::{get_connection, get_connection_pool};
use crate::mailing::mailer::render_email_context;
use crate::utils::errors_catcher::{bad_request, internal_error, not_found, unauthorized, unprocessable_entity};

mod api {
    pub mod admin {
        pub mod admin;
    }

    pub mod auth {
        pub mod signup;
        pub mod signin;
        pub mod status;
        pub mod confirm;
    }
}

mod database {
    pub mod database;
    pub mod schema;
    pub mod user;
    pub mod auth_token;
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
mod mailing {
    pub mod mailer;
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[launch]
fn rocket() -> _ {

    // migrate database
    let mut conn = get_connection();
    let res = conn.run_pending_migrations(MIGRATIONS).unwrap();
    println!("Migrations result: {:?}", res);

    rocket::build()
        .attach(cors_options())
        .manage(get_connection_pool())
        .manage(UserAgentParser::from_path("./static/user_agent_regexes.yaml").unwrap())
        .mount("/", routes![auth_signup, auth_signin, auth_status, test_template])
        .register("/", catchers![bad_request, unauthorized, not_found, unprocessable_entity, internal_error])
}

#[get("/test_template/<template>")]
fn test_template(template: &str) -> RawHtml<String> {
    let mut context = tera::Context::new();
    // context.insert("title", title);

    RawHtml(render_email_context(String::from(template), context))
}

fn cors_options() -> Cors {
    let origin = [env::var("FRONTEND_HOST").expect("FRONTEND_HOST must be set")];
    CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&origin),
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error while building CORS")
}




