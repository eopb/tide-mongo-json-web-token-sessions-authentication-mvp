use bson::doc;

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl Claims {
    pub(crate) fn new(user: String) -> Self {
        Self {
            sub: user,
            company: "test".to_owned(),
            exp: 10000000000,
        }
    }
    pub(crate) fn get_token(&self) -> String {
        let key = env::var("SECRET").unwrap();
        match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(key.as_bytes()),
        ) {
            Ok(t) => t,
            Err(_) => panic!(), // in practice you would return the error
        }
    }
}
