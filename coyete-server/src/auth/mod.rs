use std::str::FromStr;

use rwt::{Rwt, RwtError};
use self::claim::Claim;
use api::response::APIResponse;

pub mod claim;
pub mod security;

#[derive(Debug, Deserialize)]
pub struct Token(pub Rwt<Claim>);

impl Token {
    pub fn payload(&self) -> &Claim {
        &self.0.payload
    }

    // helper function to encode payload.
    pub fn inner(&self) -> &Rwt<Claim> {
        &self.0
    }

    // validates private key exists.
    pub fn is_valid(&self, secret: &[u8]) -> bool {
        self.0.is_valid(secret)
    }

    // time to live timestamp.
    pub fn ttl(&self) -> i64 {
        self.0.payload.exp.timestamp()
    }

    pub fn user(&self) -> &str {
        &self.0.payload.usr
    }
}

impl FromStr for Token {
    type Err = RwtError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserToken(s.parse::<Rwt<Claim>>()?))
    }
}

impl TokenResponse {
    pub fn from_token(token: Token) -> Result<Self, RwtError> {
        let expiration = token.timestamp();
        token
            .inner()
            .encode()
            .map(|token| TokenResponse { token, expiration })
    }
}


pub struct Authentication {
    secret: Vec<u8>,
}

impl Authentication {
    pub fn new<T: AsRef<[u8]>>(Secret: T) -> Authentication {
        Authentication { secret: Vec::from(secret.as_ref()) }
    }
}
