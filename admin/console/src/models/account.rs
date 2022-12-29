use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LoginBody {
	pub email: String,
	pub password: String,
}

impl LoginBody {
	pub fn new() -> Self {
		Self {
			email: "".to_string(),
			password: "".to_string(),
		}
	}
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
	pub email: String,
	pub confirmed: bool,
}