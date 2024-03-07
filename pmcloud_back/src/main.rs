mod grouping_strategy;
mod ftp_server;
mod database;
#[macro_use] extern crate rocket;

use diesel::migration::MigrationSource;
use diesel::mysql::Mysql;
use diesel::r2d2::Pool;
use crate::database::database::{DBPool, get_connection, get_connection_pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::prelude::*;
use crate::database::schema::{books, pages};

#[get("/")]
fn index(db: &rocket::State<DBPool>) -> String {
    let mut conn = db.get().unwrap();

    // add random momo book
    diesel::insert_into(books::table)
        .values(books::title.eq("Momo"))
        .execute(&mut conn).unwrap();

    let momo = books::table
        .filter(books::title.eq("Momo"))
        .select(Book::as_select())
        .get_result(&mut conn).unwrap();

    // get pages for a book
    let pages = Page::belonging_to(&momo)
        .select(Page::as_select())
        .load(&mut conn).unwrap();

    format!("Pages for \"Momo\": \n {}\n", momo.id)
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
        .mount("/", routes![index])
}

#[get("/world")]
fn handler() {
    println!("Hello, world!");
}


#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Book))]
#[diesel(table_name = pages)]
pub struct Page {
    pub id: i32,
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}



