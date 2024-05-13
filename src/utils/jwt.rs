
use chrono::{Duration, Local};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::constants::JWT_SECRET;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub id: String,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    pub fn new(user_id: String) -> Self {
        let iat = Local::now();
        let exp = iat + Duration::hours(24);
        Self {
            id: user_id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(id: String) -> Result<String, Error> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )?)
}

pub fn verify(token: &str) -> Result<Claims, Error> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}
