// // use axum::{
// //     async_trait,
// //     extract::{Extension, FromRequest, FromRequestParts, State},
// //     http::{header, request::Parts, HeaderMap, Response, StatusCode},
// // };
// // use serde_json::json;

// // use crate::{
// //     models::student_model::{GetUserEmail, Token},
// //     utils::api_error::Error,
// // };

// // use super::{
// //     api_error::ApiError,
// //     jwt::{self, Claims},
// // };

// // #[async_trait]
// // impl<S> FromRequestParts<S> for GetUserEmail
// // where
// //     S: Send + Sync,
// // {
// //     type Rejection = ApiError;

// //     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Claims, Self::Rejection> {
// //         // let access_token = parts
// //         //     .headers
// //         //     .get(header::AUTHORIZATION)
// //         //     .and_then(|value| value.to_str().ok());
// //         // .and_then(|str| str.split(" ").nth(1));

// //         let access_token = HeaderMap::from_request_parts(parts, state)
// //             .await
// //             .map_err(|err| match err {})?;

// //         use axum::RequestPartsExt;
// //         let Extension(state) = parts
// //             .extract::<Extension<State>>()
// //             .await
// //             .map_err(|err| err.into_response())?;

// //         println!("Access token is :  {:?}", access_token);

// //         // check if the access token is empty or not, if it is empty then return error, otherwise do operation
// //         match access_token {
// //             Some(token) => {
// //                 // get the associated id after verifying the token using the secret key
// //                 let claims = jwt::verify(token);

// //                 // checking there are any error has or not
// //                 match claims {
// //                     Ok(clam) => Ok(clam),
// //                     Err(e) => Err(Error::UnAuthorizedJwtToken)?,
// //                 }
// //             }
// //             None => Err(Error::InvalidJwtToken)?,
// //         }
// //     }
// // }

// // // // #[async_trait]
// // // // impl<B> FromRequest<B> for GetUserEmail
// // // // where
// // // //     B: Send,
// // // // {
// // // //     type Rejection = ApiError;

// // // //     async fn from_request(req: &mut FromRequestParts<B>) -> Result<Self, Self::Rejection> {
// // // //         let TypedHeader(Authorization(bearer)) =
// // // //             TypedHeader::<Authorization<Bearer>>::from_request(req)
// // // //                 .await
// // // //                 .map_err(|err| Error::from(err))?;
// // // //         let Extension(pool) = Extension::<PgPool>::from_request(req)
// // // //             .await
// // // //             .map_err(|err| Error::from(err))?;
// // // //         let claims = jwt::verify(bearer.token())?;
// // // //         Ok(User::find_by_id(claims.sub, &pool).await?)
// // // //     }
// // // // }

// // // struct Auth;

// // // #[async_trait]
// // // impl<S> FromRequestParts<S> for Auth
// // // where
// // //     S: Send + Sync,
// // // {
// // //     type Rejection = Response<String>;

// // //     async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
// // //         let access_token = parts
// // //             .headers
// // //             .get(header::AUTHORIZATION)
// // //             .and_then(|value| value.to_str().ok())
// // //             .and_then(|str| str.split(" ").nth(1));

// // //         match access_token {
// // //             Some(token) => {
// // //                 let user = jwt::verify(token);

// // //                 match user {
// // //                     Ok(user) => Ok(Auth),

// // //                     //   Err(
// // //                     // Response::builder()
// // //                     //   .status(StatusCode::UNAUTHORIZED)
// // //                     //   .header(header::CONTENT_TYPE, "application/json")
// // //                     //   .body(
// // //                     //     json!({
// // //                     //       "success": false,
// // //                     //       "data": {
// // //                     //         "message": e
// // //                     //       }
// // //                     //     })
// // //                     //     .to_string(),
// // //                     //   )
// // //                     //   .unwrap_or_default(),
// // //                     //   ),
// // //                 }
// // //             }

// // //             None => Err(Response::builder()
// // //                 .status(StatusCode::UNAUTHORIZED)
// // //                 .header(header::CONTENT_TYPE, "application/json")
// // //                 .body(
// // //                     json!({
// // //                       "success": false,
// // //                       "data": {
// // //                         "message": "No token provided"
// // //                       }
// // //                     })
// // //                     .to_string(),
// // //                 )
// // //                 .unwrap_or_default()),
// // //         }
// // //     }
// // // }

// // use axum::{
// //     async_trait,
// //     extract::{Extension, FromRequestParts},
// //     http::{request::Parts, HeaderMap, StatusCode},
// //     response::{IntoResponse, Response},
// //     routing::get,
// //     Router,
// // };

// // #[derive(Clone)]
// // struct State {
// //     // ...
// // }
// // struct AuthenticatedUser {
// //     // ...
// // }

// // #[async_trait]
// // impl<S> FromRequestParts<S> for AuthenticatedUser
// // where
// //     S: Send + Sync,
// // {
// //     type Rejection = Response;

// //     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
// //         // You can either call them directly...
// //         let headers = HeaderMap::from_request_parts(parts, state)
// //             .await
// //             .map_err(|err| match err {})?;

// //         // ... or use `extract` / `extract_with_state` from `RequestExt` / `RequestPartsExt`
// //         use axum::RequestPartsExt;
// //         let Extension(state) = parts
// //             .extract::<Extension<State>>()
// //             .await
// //             .map_err(|err| err.into_response())?;

// //         println!(" {:?}",headers);
// //         // println!(" {:?}",state);

// //         unimplemented!("actually perform the authorization")
// //     }
// // }

// use axum::{
//     async_trait,
//     extract::{Extension, FromRequest, RequestParts, TypedHeader},
// };
// use headers::{authorization::Bearer, Authorization};
// use sqlx::PgPool;

// use crate::{
//     error::{ApiError, Error},
//     model::User,
//     utils::jwt,
// };

// use super::jwt::Claims;

// #[async_trait]
// impl<B> FromRequest<B> for User
// where
//     B: Send,
// {
//     type Rejection = ApiError;

//     async fn from_request(req: &mut RequestParts<B>) -> Result<Claims, Self::Rejection> {
//         let TypedHeader(Authorization(bearer)) =
//             TypedHeader::<Authorization<Bearer>>::from_request(req)
//                 .await
//                 .map_err(|err| Error::from(err))?;
//         // let Extension(pool) = Extension::<PgPool>::from_request(req)
//         //     .await
//         //     .map_err(|err| Error::from(err))?;

//         let claims = jwt::verify(bearer.token())?;


//         Ok(User::find_by_id(claims.sub, &pool).await?)
//     }
// }
