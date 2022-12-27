use serde::{Serialize, Deserialize};

use super::Skill;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub github_name: String,
    pub devpost_link: Option<String>,
    pub project_link: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub image: Option<String>,
    pub featured: bool,
    pub skill_ids: Vec<i64>,
	pub skills: Vec<Skill>
}