use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation};
use chrono::{Utc, Duration};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize
}


pub fn create_jwt(username: &str, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &jsonwebtoken::EncodingKey::from_secret(secret))
}


pub fn verify_jwt(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )
        .map(|token| token.claims)
}