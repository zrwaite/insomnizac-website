use yew::{html, Html, function_component, Callback, MouseEvent};
use yew_router::prelude::use_navigator;

use crate::pages::Route;

#[function_component(Login)]
pub fn login() -> Html {

	let navigator = use_navigator().unwrap();
	// let signup_button: Callback<MouseEvent> = {
	// 	Callback::from(move |_| {
	// 		navigator.push(&Route::Projects);
	// 	})
    // };

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