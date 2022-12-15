use yew::{Callback, html, Html, function_component};
use yew_router::{BrowserRouter, Switch, Routable};
use yew_router::prelude::use_navigator;
use crate::pages::{Error404, Projects};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick_callback = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    html! {
        <main>
            {
                match routes {
                    Route::Home => html! { <h1>{ "Home" }</h1> },
                    Route::Secure => html! {<Secure />},
                    Route::NotFound => html! { <Error404 /> },
                    Route::Projects => html! { <Projects /> },
                }
            }
        </main>
    }
}
