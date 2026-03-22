use serde::{Serialize, Deserialize};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

pub fn encode_jwt(email: &str) -> Result<String, AppError> {
    let secret = match std::env::var("JWT_TOKEN") {
        Ok(s) => s,
        Err(_) => return Err(AppError::Internal)
    };

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp = (now + expire).timestamp() as usize;
    let iat = now.timestamp() as usize;
    let claim = Claims { iat, exp, email: email.to_string() };

    encode(&Header::default(), &claim, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|_| AppError::Internal)
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, AppError> {
    let secret = match std::env::var("JWT_TOKEN") {
        Ok(s) => s,
        Err(_) => return Err(AppError::Internal)
    };

    let result: Result<TokenData<Claims>, AppError> = decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Internal);
    result
}
