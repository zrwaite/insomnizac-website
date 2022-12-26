use yew::{html, Html, function_component, Properties};
use reqwasm::http::Request;
use yew::{use_effect_with_deps, UseStateHandle};
use yew::prelude::use_state;
use log::info;

use crate::components::EditProjectForm;
use crate::models::{Project, Skill};

#[derive(PartialEq, Properties)]
pub struct EditProjectProps {
    pub slug: String
}


#[function_component(EditProject)]
pub fn edit_project(props: &EditProjectProps) -> Html {

    let error = Box::new(use_state(|| None));

    let project: Box<UseStateHandle<Option<Project>>> = Box::new(use_state(|| None));

    let skills: Box<UseStateHandle<Vec<Skill>>> = Box::new(use_state(|| vec![]));

    {
        let project = project.clone();
        let skills = skills.clone();
        let error = error.clone();
		let slug = props.slug.clone();
        use_effect_with_deps(move |_| {
            let project = project.clone();
            let skills = skills.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let project_endpoint = format!(
                    "http://localhost:3000/projects/{}",slug.clone()
                );
                let skills_endpoint = format!("http://localhost:3000/skills");
                let fetched_project = Request::get(&project_endpoint).send().await;
                match fetched_project {
                    Ok(project_response) => {
                        let json: Result<Project, _> = project_response.json().await;
                        match json {
                            Ok(f) => {
                                info!("Success!");

                                let fetched_skills = Request::get(&skills_endpoint).send().await;
                                match fetched_skills {
                                    Ok(skills_response) => {
                                        let json: Result<Vec<Skill>, _> = skills_response.json().await;
                                        match json {
                                            Ok(s) => {
                                                info!("Success!");
                                                skills.set(s);
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
			<h1>{ format!("Edit Project: {}", props.slug.clone()) }</h1>
			<div class="projectGrid">
				{
					match (*project).as_ref() {
						Some(p) => {
                            let passed_skills = (*(*skills).clone()).clone();
							html! {
								<EditProjectForm project={(*p).clone()} skills={passed_skills}/>
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