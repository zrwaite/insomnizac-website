use yew_router::prelude::Navigator;

use crate::pages::Route;

use super::LocalStorage;

pub fn auth_redirect(navigator: Navigator) {
	let token = LocalStorage::new().unwrap().get("token".to_string());
	if token.is_none() {
		navigator.push(&Route::Login);
	}
}