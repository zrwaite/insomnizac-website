use yew::{html, Html, function_component};
use yew_router::prelude::use_navigator;


#[function_component(SignUp)]
pub fn signup() -> Html {

	let navigator = use_navigator().unwrap();
	// let signup_button: Callback<MouseEvent> = {
	// 	Callback::from(move |_| {
	// 		navigator.push(&Route::Projects);
	// 	})
    // };

    html! {
        <div>
            <h1>{ "Admin SignUp" }</h1>
			<p>
				{"Have an account? Login "}
				<a href="/login">{ "here" }</a>
			</p>
			<div class={"account_form"}>

			</div>
        </div>
    }
}