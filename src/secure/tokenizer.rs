use crate::error::Error;

use jwt_simple::prelude::*;

#[derive(Clone)]
pub struct Tokenizer {
    key: HS256Key,
    token_expiration: Duration,
}

impl Tokenizer {
    pub fn new(token_expiration: impl Into<Duration>, secret_key: Option<&str>) -> Self {
        let key = secret_key
            .filter(|secret_key| !secret_key.is_empty())
            .map(|secret_key| HS256Key::generate().with_key_id(secret_key))
            .unwrap_or_else(HS256Key::generate);

        Self {
            key,
            token_expiration: token_expiration.into(),
        }
    }

    pub fn generate(&self) -> Result<String, Error> {
        let claims = Claims::create(self.token_expiration);

        Ok(self.key.authenticate(claims)?)
    }

    pub fn verify(&self, token: &str) -> Result<String, Error> {
        Ok(self
            .key
            .verify_token::<NoCustomClaims>(&token, None)
            .map(|_| token.to_string())?)
    }
}
