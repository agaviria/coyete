pub mod claim;

use rwt::{Rwt, RwtError};
use self::claim::Claim;

use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct UserToken(pub Rwt<Claim>);

impl UserToken {
    // TODO:
    // =========================================================================
    // * Research use of private ephemeral keys instead of (secret-key).

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

    // TODO:
    // =========================================================================
    // * add `sub` struct field member of `auth::Claim` to an UserToken function.
}

impl FromStr for UserToken {
    type Err = RwtError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserToken(s.parse::<Rwt<Claim>>()?))
    }
}
