use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub id: Option<i64>,
    pub name: Option<String>
}