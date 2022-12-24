use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub image: String,
}