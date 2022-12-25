use yew::{html, Html, function_component, Properties};
use reqwasm::http::Request;
use yew::{use_effect_with_deps, UseStateHandle};
use yew::prelude::use_state;
use log::info;

use crate::models::Project;

#[derive(PartialEq, Properties)]
pub struct EditProjectProps {
    pub slug: String
}


#[function_component(EditProject)]
pub fn edit_project(props: &EditProjectProps) -> Html {

    let error = Box::new(use_state(|| None));

    let project: Box<UseStateHandle<Option<Project>>> = Box::new(use_state(|| None));

    {
        let project = project.clone();
        let error = error.clone();
		let slug = props.slug.clone();
        use_effect_with_deps(move |_| {
            let project = project.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let project_endpoint = format!(
                    "http://localhost:3000/projects/{}",slug.clone()
                );
                let fetched_project = Request::get(&project_endpoint).send().await;
        
                match fetched_project {
                    Ok(response) => {
                        let json: Result<Project, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                info!("Success!");
                                project.set(Some(f));
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
					match (*project).as_ref() {
						Some(p) => {
							html! {
								<div>
									<h1>{ p.name.clone() }</h1>
								</div>
							}
						}
						None => {
							html! {
								<div>
									<h1>{ "Loading..." }</h1>
								</div>
							}
						}
					}
				}
			</div>
		</div>
    }
}