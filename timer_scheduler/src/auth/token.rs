use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

const SECRET: &[u8] = b"my_secret_key"; // ⚠️ move to env variable in real app

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_token(user_name: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24)) // valid for 24h
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_name.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    ).expect("Token creation failed")
}
