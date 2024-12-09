use std::env;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

// const SECRET_KEY: &[u8] = b"your_secret_key";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub email: String,
    pub name: String,
    pub exp: usize,
}

pub fn generate_jwt(user_id: &i32, email: &str, name: &str) -> String {
    let jwt_duration = env::var("JWT_DURATION").unwrap_or(String::from("24"));
    let duration: i64;
    match jwt_duration.parse::<i64>() {
        Ok(parsed_duration) => duration = parsed_duration,
        Err(_) => duration = 24
    }
    
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(duration))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        email: email.to_owned(),
        name: name.to_owned(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .expect("Failed to encode JWT")
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
