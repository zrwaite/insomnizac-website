use yew::{html, Html, function_component};
use reqwasm::http::Request;
use yew::{use_effect_with_deps, UseStateHandle};
use yew::prelude::use_state;
use log::info;

use crate::models::Project;
use crate::components::ProjectPanel;

#[function_component(Projects)]
pub fn projects() -> Html {

    let error = Box::new(use_state(|| None));

    let projects: Box<UseStateHandle<Vec<Project>>> = Box::new(use_state(|| vec![]));

    {
        let projects = projects.clone();
        let error = error.clone();
        use_effect_with_deps(move |_| {
            let projects = projects.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let projects_endpoint = format!(
                    "http://localhost:3000/projects"
                );
                let fetched_projects = Request::get(&projects_endpoint).send().await;
        
                match fetched_projects {
                    Ok(response) => {
                        let json: Result<Vec<Project>, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                info!("Success!");
                                projects.set(f);
                            }
                            Err(e) => {
                                info!("Error! {}", e.to_string());
                                error.set(Some(e.to_string()));
                            }
                        }
                    }
                    Err(e) => {
                        info!("Error! {}", e.to_string());
                        error.set(Some(e.to_string()))
                    }
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