use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};
use diesel::prelude::*;
use crate::database::database::DBPool;
use crate::database::schema::users;
use crate::database::user::User;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // get user_id and auth_token from request headers
        let user_id = request.headers().get_one("X-User-Id").map(|s| s.parse::<u32>().ok()).flatten();
        let auth_token = request.headers().get_one("X-Auth-Token").map(|s| hex::decode(s).ok()).flatten();

        if user_id.is_none() || auth_token.is_none() {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let db: &DBPool = request.rocket().state::<DBPool>().unwrap();
        let conn = &mut db.get().unwrap();

        let user = users::dsl::users
            .filter(users::dsl::id.eq(user_id.unwrap()))
            .filter(users::dsl::auth_token.eq(auth_token.unwrap()))
            .first(conn);

        if user.is_err() {
            return Outcome::Error((Status::Unauthorized, ()));
        }
        Outcome::Success(user.unwrap())
    }
}
