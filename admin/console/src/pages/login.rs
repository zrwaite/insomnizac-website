use log::info;
use web_sys::MouseEvent;
use yew::{html, Html, function_component, Callback, use_state};

use crate::{utils::{http_request, HttpResponse}, models::{User, LoginBody}};
// use yew_router::prelude::use_navigator;

#[function_component(Login)]
pub fn login() -> Html {
    let error = Box::new(use_state(|| None));
	let login_body_state = Box::new(use_state(|| LoginBody::new()));

	// let navigator = use_navigator().unwrap();
	// let signup_button: Callback<MouseEvent> = {
	// 	Callback::from(move |_| {
	// 		navigator.push(&Route::Projects);
	// 	})
    // };

	let login_button: Callback<MouseEvent> = {
		let error = error.clone();
		Callback::from(move |_| {
			let error = error.clone();
			wasm_bindgen_futures::spawn_local(async move {
				match http_request::<User>(
					"http://localhost:3000//users/login".to_string(), 
					crate::utils::HttpMethod::POST, 
					Some(serde_json::to_string(&"".to_string()).unwrap())
				).await {
					HttpResponse::Success(_p) => {info!("Success!")},
					HttpResponse::Error(e) => {
						error.set(Some(
							format!("Error: {}, {}, {}", e.status, e.error, e.exception)
						));
					}
					HttpResponse::Unknown(e) => {
						error.set(Some(e.to_string()));
					}
				}
			});
		})
    };

    html! {
        <div>
            <h1>{ "Admin Login" }</h1>
			<p>
				{"Don't have an account? Create one "}
				<a href="/signup">{ "here" }</a>
			</p>
			<div class={"account_form"}>
				<div>
					<label for="email">{"Email: "}</label>
					<input type="email" name="email" />
				</div>
				<div>
					<label for="password">{"Password: "}</label>
					<input type="password" name="password" />
				</div>
			</div>
        </div>
    }
}