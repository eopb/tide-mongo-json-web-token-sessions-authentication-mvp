//! Shared application state.

use std::env;

#[derive(Debug)]
pub(crate) struct State {
    pub db: mongodb::Client,
}

impl State {
    /// Create a new instance of `State`.
    pub(crate) async fn new() -> tide::Result<Self> {
        let mongo = mongodb::Client::with_uri_str(&env::var("DB_URL").unwrap()).await?;
        Ok(Self { db: mongo })
    }

    /// Access the mongodb client.
    pub(crate) fn mongo(&self) -> &mongodb::Client {
        &self.db
    }
    fn testing_ground(&self) -> mongodb::Database {
        let name: String = "testing-ground".to_string();
        log::debug!("accessing database {}", name);
        self.mongo().database(&name)
    }

    pub(crate) fn users(&self) -> mongodb::Collection {
        let name: String = "users".to_string();
        log::debug!("accessing collection {}", name);
        self.testing_ground().collection(&name)
    }
}
