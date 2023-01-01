use yew::{html, Html, function_component};
use yew::{use_effect_with_deps};
use yew::prelude::use_state;
use log::info;
use yew_router::prelude::use_navigator;

use crate::models::Project;
use crate::components::ProjectPanel;
use crate::utils::{HttpResponse, get_request, auth_redirect};

#[function_component(Projects)]
pub fn projects() -> Html {
	let navigator = use_navigator().unwrap();
    auth_redirect(navigator);
    let error = Box::new(use_state(|| None));
    let projects = Box::new(use_state(|| vec![]));

    {
        let projects = projects.clone();
        let error = error.clone();
        use_effect_with_deps(move |_| {
            let projects = projects.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                match get_request::<Vec<Project>>(
                    "http://localhost:3000/projects".to_string()
                ).await {
                    HttpResponse::Success(p) => {
                        info!("Success!");
                        projects.set(p);
                    },
                    HttpResponse::Error(e) => error.set(Some(
                        format!("Error: {}, {}, {}", e.status, e.error, e.exception)
                    )),
                    HttpResponse::Unknown(e) => error.set(Some(e.to_string()))
                }
            });
            || ()
        }, ());
    }
    
    html! {
        <div>
            <h1>{ "Projects Page" }</h1>
            <div class="projectGrid">
                {
                    for projects.iter().map(|project| {
                        html!{
                            <ProjectPanel project={project.clone()}/>
                        }
                    })
                }
            </div>
        </div>
    }
}