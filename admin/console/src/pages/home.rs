use yew::{html, Html, function_component, Callback, MouseEvent};
use yew_router::prelude::use_navigator;

use crate::pages::Route;

#[function_component(Home)]
pub fn home() -> Html {

	let navigator = use_navigator().unwrap();
	let projects_button: Callback<MouseEvent> = {
		Callback::from(move |_| {
			navigator.push(&Route::Projects);
		})
    };

    html! {
        <div>
            <h1>{ "Insomnizac Admin" }</h1>
			<button onclick={projects_button}>{ "Projects" }</button>
        </div>
    }
}