use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct LoginBody {
	pub email: String,
	pub password: String,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
	pub email: String,
	pub confirmed: bool,
}