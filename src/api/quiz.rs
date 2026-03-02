
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use crate::api::DB;


// structs
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quiz {
    pub name: String,
    pub questions: Vec<Question>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub q: String,
    pub answers: Vec<String>,
    pub correct: i64,
}


// methods


#[server]
pub async fn quizzes_get() -> Result<Vec<Quiz>> {

    // let db = DB.read().unwrap().quizzes.clone()



    Ok(DB.read().expect("wah").quizzes.clone())
}


#[server]
pub async fn quiz_get(id: u32) -> Result<Quiz> {
    let db = DB.read().unwrap();
    Ok(db.quizzes[id as usize].clone())
}
