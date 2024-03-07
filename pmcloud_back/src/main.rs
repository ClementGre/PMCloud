mod grouping_strategy;
mod ftp_server;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


#[get("/world")]
fn handler() {


}



