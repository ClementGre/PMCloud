use std::fmt::Display;

use async_trait::async_trait;
use libunftp::auth::{AuthenticationError, Authenticator, Credentials, UserDetail};

#[derive(Debug)]
pub struct PMAuthenticator;

#[async_trait]
impl Authenticator<PMUser> for PMAuthenticator {
    async fn authenticate(&self, _username: &str, _creds: &Credentials) -> Result<PMUser, AuthenticationError> {
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
