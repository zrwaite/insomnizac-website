use yew::{html, Html, function_component};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "Insomnizac Admin" }</h1>
			<a href="/projects">{ "Projects" }</a><br/>
			<a href="/login">{ "Login" }</a><br/>
			<a href="/signup">{ "Sign Up" }</a>
        </div>
    }
}