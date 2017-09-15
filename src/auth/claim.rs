use chrono::Duration;
use chrono::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use rwt::RwtError;
use service;

use std::str::FromStr;

#[derive(Debug)]
pub struct Claim {
    // time expiration
    pub exp: DateTime<Utc>,
    // user id token issued for
    pub sub: i64,
    // bearer user
    pub usr: String,
}

impl Claim {
    fn new<T: Into<String>>(sub: i64, usr: T) -> Claim {
        Claim {
            exp: Utc::now() + Duration::seconds(1800),
            sub,
            usr: usr.into(),
        }
    }

    fn is_valid(&self) -> bool {
        Utc::now() < self.exp
    }
}

impl FromStr for Claim {
    type Err = RwtError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use serde_json;
        Ok(serde_json::from_str(s).map_err(RwtError::Json)?)
    }
}

impl Serialize for Claim {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Template<'a> {
            exp: i64,
            sub: String,
            usr: &'a str,
        }

        let template = Template {
            exp: self.exp.timestamp(),
            sub: service::harsh().encode(&[self.sub as u64]).unwrap(),
            usr: &self.usr,
        };

        template.serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for Claim {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        fn from_timestamp(expires: i64) -> Option<DateTime<Utc>> {
            NaiveDateTime::from_timestamp_opt(expires, 0).map(|datetime| {
                                                                  Utc.from_utc_datetime(&datetime)
                                                              })
        }

        #[derive(Deserialize)]
        struct Template {
            exp: i64,
            sub: String,
            usr: String,
        }

        let Template { exp, sub, usr } = Template::deserialize(deserializer)?;
        let exp = from_timestamp(exp).ok_or_else(|| Error::custom("Invalid timestamp"))?;
        let sub = *service::harsh().decode(&sub)
                       .unwrap()
                       .first()
                       .ok_or_else(|| Error::custom("Invalid user id"))? as i64;

        Ok(Claim { exp, sub, usr })
    }
}
