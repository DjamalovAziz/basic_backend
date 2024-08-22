pub use std::{collections::HashMap, env::vars};

use actix_web::http::header::HeaderValue;
use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use chrono::{Duration, SubsecRound, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub async fn generate_token(data: String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now().trunc_subsecs(0);
    let claims = Claims {
        iat: now.timestamp() as usize,
        exp: (now + Duration::weeks(100000)).timestamp() as usize,
        sub: data,
    };

    let access_token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_env_or("SECRET_KEY", "BASIC").as_ref()),
    )?;
    Ok(access_token)
}

pub fn decode_token(token: &HeaderValue) -> Result<String, jsonwebtoken::errors::Error> {
    let split: Vec<&str> = token.to_str().unwrap_or_default().split("Bearer").collect();

    let access_token = split[1].trim();

    let key = get_env_or("SECRET_KEY", "BASIC");

    match decode::<Claims>(
        access_token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token) => Ok(token.claims.sub),
        Err(e) => Err(e),
    }
}

pub fn generate_hash(password: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let argon2 = Argon2::default();

    Ok(argon2
        .hash_password(password, &SaltString::generate(&mut OsRng))?
        .to_string())
}

pub fn verify_password_hash(
    data_password: String,
    hashed_password: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let parsed_hash = PasswordHash::new(&hashed_password)?;
    Ok(argon2::Argon2::default()
        .verify_password(data_password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn send_sms(phone_number: &str, text: String) -> Result<bool, reqwest::Error> {
    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::AUTHORIZATION,
        format!("Basic {}", base64::encode("wellnor:f1DZ#KymW"))
            .parse()
            .expect("send_sms"),
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    let payload = json!({
        "messages": [{
            "recipient": phone_number.replace(|c: char| !c.is_ascii_digit(), ""),
            "message-id": format!("{}_{}", phone_number, chrono::Local::now().format("%Y%m%d_%H%M%S")),
            "sms": {
                "originator": "3700",
                "content": {
                    "text": text
                }
            }
        }]
    });

    let client = reqwest::Client::new();

    Ok(client
        .post("http://91.204.239.44/broker-api/send")
        .headers(headers)
        .json(&payload)
        .send()
        .await?
        .status()
        .is_success())
}

pub fn get_env_or(key: &str, default: &str) -> String {
    let env: HashMap<String, String> = vars().collect();
    match env.get(key) {
        Some(value) => value.to_string(),
        _ => default.to_string(),
    }
}
