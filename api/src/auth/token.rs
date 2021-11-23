use crate::result::Result;

use data::models::{Group, User};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: Vec<Group>,
    exp: usize,
}

impl Claims {
    pub fn for_user(
        user: User,
        groups: Vec<Group>,
        valid_for: i64,
    ) -> Result<Claims> {
        Ok(Claims {
            sub: user.username,
            aud: groups,
            exp: (chrono::Utc::now() + chrono::Duration::seconds(valid_for))
                .timestamp() as usize,
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
}
