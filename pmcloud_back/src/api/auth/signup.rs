use rocket::serde::{Deserialize, json::Json};
use crate::database::database::DBPool;


#[derive(Deserialize)]
struct SignupData {
    name: String,
    email: String,
    password: String,
}

#[get("/auth/signup", data = "<data>")]
pub fn auth_signup(data: Json<SignupData>, db: &rocket::State<DBPool>) -> String {

    String::from(data.name.clone())
}
