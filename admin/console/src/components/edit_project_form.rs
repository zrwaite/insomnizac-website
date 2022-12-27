use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use yew::{function_component, Html, Properties, html, Callback, MouseEvent, UseStateHandle, use_state};
use reqwasm::http::{Request, Response};
use log::info;

use crate::{models::{Project, Skill, RailsError}};

#[derive(PartialEq, Properties)]
pub struct ProjectPanelProps {
    pub project: Project,
	pub skills: Vec<Skill>
}


#[function_component(EditProjectForm)]
pub fn edit_project_form(props: &ProjectPanelProps) -> Html {
    let ProjectPanelProps { project, skills } = props;
	let slug = project.slug.clone();
    let error = Box::new(use_state(|| None));
    let project_skills = Box::new(use_state(|| project.skills.clone()));
	let new_skill: Box<UseStateHandle<Option<Skill>>> = Box::new(use_state(|| None));
	let parsed_project_skills = (*(*project_skills).clone()).clone();
	let unused_skills = skills.to_vec().into_iter().filter(|s| 
		!parsed_project_skills.iter().any(|ps| ps.id == s.id)
	).collect::<Vec<Skill>>();

	let save_button: Callback<MouseEvent> = {
		let slug = slug.clone();
		let error = error.clone();
		let saved_project = project.clone();
		let parsed_project_skills = parsed_project_skills.clone();
		Callback::from(move |_| {
			let slug = slug.clone();
			let mut saved_project = saved_project.clone();
			saved_project.skill_ids = parsed_project_skills.clone().into_iter().map(|s| s.id).collect();
			let error = error.clone();
			wasm_bindgen_futures::spawn_local(async move {
                let projects_endpoint = format!(
                    "http://localhost:3000/projects/{}", slug.clone()
                );
                let mut update_request = Request::put(&projects_endpoint);
				// update_request.header("Content-Type", "application/json");
				update_request = update_request.body(serde_json::to_string(&saved_project).unwrap());

				let update_response = update_request.send().await;
        
                match update_response {
                    Ok(response) => {
                        let text = response.text().await.unwrap();
						let project_response: Result<Project, _> = serde_json::from_str(&text.clone());
						let error_response: Result<RailsError, _> = serde_json::from_str(&text.clone());
                        match project_response {
                            Ok(p) => {
                                info!("Success!");
                            }
                            Err(_) => {
                                match error_response {
									Ok(e) => {
										error.set(Some(
											format!("Error: {}, {}, {}", e.status, e.error, e.exception)
										));
									}
									Err(e) => {
										error.set(Some(
											format!("Unknown Error: {}", e.to_string())
										));
									}
								}
                            }
                        }
                    }
                    Err(e) => {
                        info!("Error! {}", e.to_string());
                        error.set(Some(format!("Error! {}", e.to_string())));
                    }
                }
            });
		})
    };

	let add_skill_button: Callback<MouseEvent> = {
		let parsed_project_skills = parsed_project_skills.clone();
		let project_skills = project_skills.clone();
		let new_skill = new_skill.clone();
		Callback::from(move |_| {
			let mut new_skills = parsed_project_skills.clone();
			let parsed_new_skill = (*(*new_skill).clone()).clone().unwrap();
			new_skills.push(parsed_new_skill.clone());
			project_skills.set(new_skills.to_vec());
			new_skill.set(None);
		})
	};

	html! {
		<div class="edit_project">
			<div>
				<label for="name">{"Name: "}</label>
				<input type="text" name="name" value={project.name.clone()}/>
			</div>
			<div>
				<label for="image">{"Image: "}</label	>
				<img src={project.image.to_owned()}/>
			</div>
			<div>
				<label for="devpost_link">{"Devpost Link: "}</label>
				<input type="text" name="devpost_link" value={project.devpost_link.clone()}/>
			</div>
			<div>
				<label for="project_link">{"Project Link: "}</label>
				<input type="text" name="project_link" value={project.project_link.clone()}/>
			</div>
			<div>
				<label for="featured">{"Featured: "}</label>
				<input type="checkbox" name="featured" checked={project.featured.clone()}/>
			</div>
			<div>
				<label for="skills">{"Skills: "}</label>
				<ul>
				{for parsed_project_skills.clone().iter().map(|skill| {
					let project_skills = project_skills.clone();
					let delete_skill_button: Callback<MouseEvent> = {
						let parsed_project_skills = parsed_project_skills.clone();
						let project_skills = project_skills.clone();
						let skill = skill.clone();
						Callback::from(move |_| {
							let new_skills: Vec<Skill> = parsed_project_skills.clone().into_iter().filter(|s| s.id != skill.id.clone()).collect();
							project_skills.set(new_skills)
						})
					};
					html! {
						<li>
							<img src={skill.image.to_owned()}/>
							<p>{skill.name.to_owned()}</p>
							<button onclick={delete_skill_button}>{"Delete"}</button>
						</li>
					}
				})}
				</ul>
				<div>
					<label for="new_skill">{"New Skill: "}</label>
					<select name="new_skill" >
						<option disabled=true selected=true>{"Select a skill"}</option>
						{for (*unused_skills).to_vec().iter().map(|skill| {
							let new_skill = new_skill.clone();
							let skill_copy = skill.clone();
							let skill = skill.clone();
							html! {
								<option value={skill.name.to_owned()} onclick={Callback::from(move |_| {
									let new_skill = new_skill.clone();
									let skill = skill.clone();
									new_skill.set(Some(skill.clone()))
								})}>{skill_copy.name.clone()}</option>
							}
						})}

					</select>
					// match (*project).as_ref() {
					{if let Some(_skill) = (*new_skill).as_ref() {
						html! {
							<button onclick={add_skill_button}>{"Add Skill"}</button>
						}
					} else {
						html! {}
					}}
				</div>

			</div>
			<div class="description">{"Created At: "}{project.created_at.to_owned()}</div>
			<div class="description">{"Updated At: "}{project.updated_at.to_owned()}</div>
			<button onclick={save_button}>{"Save"}</button>
			{if let Some(error) = (*error).as_ref() {
				html! {
					<div class="error">{error.to_owned()}</div>
				}
			} else {
				html! {}
			}}
		</div>
	}
}