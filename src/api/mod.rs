use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use crate::api::quiz::Quiz;


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







