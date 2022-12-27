use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RailsError {
    pub status: i64,
	pub error: String,
	pub exception: String,
}