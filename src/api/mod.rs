#[cfg(feature = "server")]
use crate::api::quiz::Quiz;
#[cfg(feature = "server")]
use lazy_static::lazy_static;
#[cfg(feature = "server")]
use serde::Deserialize;
#[cfg(feature = "server")]
use serde::Serialize;
#[cfg(feature = "server")]
use std::sync::{Arc, RwLock};

pub mod quiz;

// The database is only available to server code
#[cfg(feature = "server")]
lazy_static! {
    static ref DB: Arc<RwLock<Database>> = {
        // Open the database from the persisted "hotdog.db" file
       Arc::new(RwLock::new(serde_json::from_slice(&std::fs::read("db.jsonc").unwrap()).unwrap()))
    };
}

#[cfg(feature = "server")]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Database {
    pub quizzes: Vec<Quiz>,
}
