use crate::result::{ApiError, Result};

use data::models::{Group, User};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub username: String,

    #[serde(rename = "aud")]
    pub groups: Vec<Group>,

    #[serde(rename = "iat")]
    pub issued_at: i64,

    #[serde(rename = "exp")]
    pub expires_at: i64,
}

impl Claims {
    pub fn for_user(
        user: User,
        groups: Vec<Group>,
        valid_for: i64,
    ) -> Result<Claims> {
        Ok(Claims {
            username: user.username,
            groups,
            issued_at: chrono::Utc::now().timestamp(),
            expires_at: (chrono::Utc::now()
                + chrono::Duration::seconds(valid_for))
            .timestamp(),
        })
    }

    pub fn encode(&self, pk: &Vec<u8>) -> Result<String> {
        Ok(jsonwebtoken::encode(
            &Header::new(Algorithm::ES256),
            &self,
            &EncodingKey::from_ec_pem(pk)?,
        )?)
    }

    pub fn decode(token: &str, pk: &Vec<u8>) -> Result<Claims> {
        Ok(jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_ec_pem(&pk)?,
            &Validation::default(),
        )?
        .claims)
    }

    pub fn from_bearer(token: &str, pk: &Vec<u8>) -> Result<Claims> {
        if !token.to_lowercase().starts_with("bearer ") {
            return Err(ApiError::TokenError(
                jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::InvalidToken,
                ),
            ));
        }

        Self::decode(&token[6..], pk)
    }
}
