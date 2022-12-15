use yew::{html, Html, function_component};

#[function_component(Error404)]
pub fn error404() -> Html {
    html! {
        <div>
            <h1>{ "404 Not Found" }</h1>
        </div>
    }
}