use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use worker::*;
use worker_kv::KvStore;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct Authenticator {
    key: EncodingKey,
    users: KvStore,
}

impl Authenticator {
    pub fn new(env: &Env) -> Result<Authenticator> {
        let pem = env.secret("AUTH_PRIVATE_KEY")?.to_string();
        let key = EncodingKey::from_rsa_pem(pem.as_bytes())
            .map_err(|_| Error::RustError("Could not sign token!".to_string()))?;
        let users = env.kv("USED_NAMES_KV")?;
        Ok(Authenticator { key, users })
    }

    pub async fn generate_token(&self, username: String) -> Result<String> {
        let claims = Claims {
            sub: username,
            exp: Utc::now()
                .checked_add_signed(Duration::days(1))
                .unwrap()
                .timestamp() as usize,
        };

        let token = encode(&Header::default(), &claims, &self.key)
            .map_err(|_| Error::RustError("Could not sign token!".to_string()))?;

        Ok(token)
    }
}
