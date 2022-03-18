use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Species {
    pub id: Option<i64>,
    pub name: Option<String>
}