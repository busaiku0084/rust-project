use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: usize,
    pub name: String,
    pub message: String,
    pub timestamp: String,
}
