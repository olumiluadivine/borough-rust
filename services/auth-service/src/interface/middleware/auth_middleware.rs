// use actix_web::{dev::ServiceRequest, Error, HttpMessage};
// use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
// use actix_web_httpauth::extractors::AuthenticationError;
// use shared::auth::{SecurityHelper, JwtClaims};
//
// pub async fn jwt_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     let security = credentials.security();
//
//     // TODO: Get JWT secret from configuration
//     let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
//
//     match SecurityHelper::validate_jwt(security, &jwt_secret) {
//         Ok(claims) => {
//             req.extensions_mut().insert(claims);
//             Ok(req)
//         }
//         Err(_) => {
//             let config = Config::default();
//             Err(AuthenticationError::from(config).into())
//         }
//     }
// }
//
// pub async fn optional_jwt_validator(req: ServiceRequest, credentials: Option<BearerAuth>) -> Result<ServiceRequest, Error> {
//     if let Some(credentials) = credentials {
//         jwt_validator(req, credentials).await
//     } else {
//         Ok(req)
//     }
// }
