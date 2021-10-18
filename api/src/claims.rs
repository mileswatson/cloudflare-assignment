use chrono::{Duration, Utc};
use worker::*;

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn generate_token(username: String) -> Result<String> {
        let claim = Claims {
            sub: username,
            exp: Utc::now()
                .checked_add_signed(Duration::hours(24))
                .unwrap()
                .timestamp() as usize,
        };
        encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .map_err(|_| Error::RustError("Could not create token!".to_string()))
    }
}
