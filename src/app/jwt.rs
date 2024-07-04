use std::str::FromStr;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{error::try_load_dot_env, JWT_SECRET};

// There isnt much reason for these to be a usize unless its forced by the
// encoding
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    pub email: String,
    pub id: i64,
}

impl Claims {
    pub fn encode(self) -> crate::Result<String> {
        try_load_dot_env();
        let secret = crate::env_or_error(JWT_SECRET)?;

        Ok(encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        )?) // coerce to our error
    }

    pub fn decode(s: impl AsRef<str>) -> crate::Result<Self> {
        s.as_ref().parse() // Calls the FromStr impl for Claims
    }
}

type Email = String;
type Id = i32;

impl TryFrom<(Email, Id)> for Claims {
    type Error = crate::Error;

    fn try_from((email, id): (Email, Id)) -> Result<Self, Self::Error> {
        let now = Utc::now();
        let Some(exp) = now.checked_add_signed(Duration::minutes(60)) else {
            panic!("time has overflown, the world is ending in one hour")
        };

        Ok(Self {
            exp: exp.timestamp(),
            iat: now.timestamp(),
            id: id.into(),
            email,
        })
    }
}

impl FromStr for Claims {
    type Err = crate::Error;

    fn from_str(jwt: &str) -> Result<Self, Self::Err> {
        try_load_dot_env();
        let secret = crate::env_or_error(JWT_SECRET)?;

        let de = decode::<Self>(
            jwt,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;
        Ok(de.claims)
    }
}
