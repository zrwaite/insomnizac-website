use log::info;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, EventTarget, HtmlInputElement};
use yew::{Html, function_component, Callback, use_state};
use yew::html;
use yew::html::{onchange::Event};
use yew_router::prelude::use_navigator;

use crate::models::account::LoginResponse;
use crate::models::LoginBody;
use crate::pages::Route;
use crate::utils::{http_request, HttpResponse, parse_state, LocalStorage, non_auth_redirect};

#[function_component(Login)]
pub fn login() -> Html {
    let error = Box::new(use_state(|| None));
	let login_body_state = Box::new(use_state(|| LoginBody::new()));
	let login_body = parse_state(login_body_state.clone());
	let navigator = use_navigator().unwrap();
	non_auth_redirect(navigator.clone());
	
	let login_button: Callback<MouseEvent> = {
		let navigator = navigator.clone();
		let error = error.clone();
		let login_body = login_body.clone();
		Callback::from(move |_| {
			let error = error.clone();
			let navigator = navigator.clone();
			let login_body = login_body.clone();
			wasm_bindgen_futures::spawn_local(async move {
				info!("Logging in...");
				info!("Body: {:?}", login_body);
				match http_request::<LoginResponse>(
					"http://localhost:3000/users/login".to_string(), 
					crate::utils::HttpMethod::POST, 
					Some(serde_json::to_string(&login_body).unwrap())
				).await {
					HttpResponse::Success(p) => {
						info!("Success!");
						LocalStorage::new().unwrap().set("token".to_string(), p.token).unwrap();
						navigator.push(&Route::Projects);
					},
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

	let on_text_change = {
		let login_body = login_body.clone();
		Callback::from(move |e: Event| {
			let mut login_body = login_body.clone();
			let target: EventTarget = e
				.target()
				.expect("Event should have a target when dispatched");
			let name = target.clone().unchecked_into::<HtmlInputElement>().name();

			match name.as_str() {
				"email" => login_body.email = target.unchecked_into::<HtmlInputElement>().value(),
				"password" => login_body.password = target.unchecked_into::<HtmlInputElement>().value(),
				_ => ()
			}
			login_body_state.set(login_body);
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
					<input type="email" name="email" onchange={on_text_change.clone()} />
				</div>
				<div>
					<label for="password">{"Password: "}</label>
					<input type="password" name="password" onchange={on_text_change.clone()} />
				</div>
			</div>
			<button onclick={login_button}>{ "Login" }</button>
			{
				if let Some(e) = (*error).as_ref() {
					html! {
						<p>{e}</p>
					}
				} else {
					html! {}
				}
			}
        </div>
    }
}