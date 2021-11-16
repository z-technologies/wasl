use crate::result::Result;

use business::config::get_environment_value;
use business::security::crypto::asymmetric::load_file_bytes;
use data::models::{Group, User};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    aud: Vec<Group>,
    exp: usize,
}

impl Claims {
    pub fn for_user(user: &User, groups: Vec<Group>) -> Result<Claims> {
        let token_validity: i64 =
            get_environment_value("AUTH_TOKEN_EXP_SECONDS")?
                .parse()
                .expect("invalid token validity seconds value");

        Ok(Claims {
            sub: user.username.clone(),
            aud: groups,
            exp: (chrono::Utc::now()
                + chrono::Duration::seconds(token_validity))
            .timestamp() as usize,
        })
    }

    pub fn encode(&self) -> Result<String> {
        let private_key =
            load_file_bytes(&get_environment_value("AUTH_PRIVATE_KEY")?)?;

        Ok(jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_ec_pem(&private_key)?,
        )?)
    }

    pub fn decode(token: &str) -> Result<Claims> {
        let private_key =
            load_file_bytes(&get_environment_value("AUTH_PRIVATE_KEY")?)?;

        Ok(jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_ec_pem(&private_key)?,
            &Validation::default(),
        )?
        .claims)
    }
}
