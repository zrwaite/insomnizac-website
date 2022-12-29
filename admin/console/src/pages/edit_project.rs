use yew::{html, Html, function_component, Properties};
use yew::{use_effect_with_deps, UseStateHandle};
use yew::prelude::use_state;
use log::info;

use crate::components::EditProjectForm;
use crate::models::{Project, Skill};
use crate::utils::{HttpResponse, get_request};

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
                match get_request::<Project>(
					format!("http://localhost:3000/projects/{}", slug.clone()), 
				).await {
					HttpResponse::Success(p) => {
                        match get_request::<Vec<Skill>>(
                            "http://localhost:3000/skills".to_string()
                        ).await {
                            HttpResponse::Success(s) => {
                                info!("Success!");
                                skills.set(s);
                                project.set(Some(p));
                            },
                            HttpResponse::Error(e) => error.set(Some(
                                format!("Error: {}, {}, {}", e.status, e.error, e.exception)
                            )),
                            HttpResponse::Unknown(e) => error.set(Some(e.to_string()))
                        }
                    },
					HttpResponse::Error(e) => error.set(Some(
                        format!("Error: {}, {}, {}", e.status, e.error, e.exception)
                    )),
					HttpResponse::Unknown(e) => error.set(Some(e.to_string())),
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