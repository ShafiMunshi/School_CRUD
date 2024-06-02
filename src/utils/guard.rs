use crate::utils::jwt;

use super::api_error::Error;
use axum::{extract::Request, http::header::AUTHORIZATION, middleware::Next, response::Response};

use super::api_error::ApiResult;

pub async fn guard(mut req: Request, next: Next) -> ApiResult<Response> {
    // get the token
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(|| return Error::InvalidJwtToken);

    match token {
        Ok(val) => {
            let main_header = val.to_str();

            let bearer_token = match main_header {
                Ok(value) => value[7..].to_string(), // we need remove "Bearer " ( 7 words from the encoded string)
                Err(e) => {
                    // we willl return Error on response -->  if something error happens
                    println!("{:?}", e);
                    String::from("")
                }
            };

            println!("{}", bearer_token);

            // after getting the bearer token, we need to verify that
            let claim = jwt::verify(&bearer_token);

            match claim {
                Ok(claims) => {
                    println!("user id is:  {}", claims.id);
                    req.extensions_mut().insert(claims);
                    Ok(next.run(req).await)
                }
                Err(_) => Err(Error::InvalidJwtToken)?,
            }
        }
        Err(e) => Err(Error::InvalidJwtToken)?,
    }
}
