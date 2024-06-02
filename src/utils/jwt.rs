
use chrono::{Duration, Local};
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::constants::JWT_SECRET;

/// Token Payload data for Jwt Authentcation
/// 

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Claims {
    pub id: String,
    pub iat: i64,
    pub exp: i64,// required
}

impl Claims {
    pub fn new(user_id: String) -> Self {
        /// token creation date time
        let iat = Local::now();
        /// token expire date time 
        let exp = iat + Duration::hours(24);
        Self {
            id: user_id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

/// create a jwt token
pub fn sign(id: String) -> Result<String, Error> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )?)
}

/// verify the jwt token and extract the payload data
pub fn verify(token: &str) -> Result<Claims, Error> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}
