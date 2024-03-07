use std::fmt::Display;
use libunftp::auth::{Authenticator, AuthenticationError, UserDetail};
use async_trait::async_trait;

#[derive(Debug)]
pub struct PMAuthenticator;

#[async_trait]
impl Authenticator<PMUser> for PMAuthenticator {
    async fn authenticate(&self, _username: &str, _password: &str) -> Result<PMUser, AuthenticationError> {
        Ok(PMUser{})
        //Ok(AuthenticationError::BadPassword)
    }
}

#[derive(Debug)]
pub struct PMUser;

impl UserDetail for PMUser {}

impl Display for PMUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RandomUser")
    }
}
