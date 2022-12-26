use yew::{function_component, Html, Properties, html, Callback, MouseEvent, UseStateHandle, use_state};
use reqwasm::http::Request;
use log::info;

use crate::{models::{Project, Skill}};

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

	let save_button: Callback<MouseEvent> = {
		let slug = slug.clone();
		let project = project.clone();
		Callback::from(move |_| {
			let slug = slug.clone();
			let project = project.clone();
			let error = error.clone();
			wasm_bindgen_futures::spawn_local(async move {
                let projects_endpoint = format!(
                    "http://localhost:3000/projects/{}", slug.clone()
                );
                let fetched_projects = Request::post(&projects_endpoint).send().await;
        
                match fetched_projects {
                    Ok(response) => {
                        let json: Result<Project, _> = response.json().await;
                        match json {
                            Ok(p) => {
                                info!("Success!");
                                // project_state.set(Some(p));
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
		})
    };

	let add_skill_button: Callback<MouseEvent> = {
		let project_skills = project_skills.clone();
		let new_skill = new_skill.clone();
		Callback::from(move |_| {
			let mut new_skills = (*(*project_skills.clone()).clone()).clone();
			let parsed_new_skill = (*(*new_skill).clone()).clone().unwrap();
			new_skills.push(parsed_new_skill.clone());
			project_skills.set(new_skills.to_vec());
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
				{for project_skills.iter().map(|skill| {
					let delete_skill_button: Callback<MouseEvent> = {
						let project_skills = project_skills.clone();
						let skill = skill.clone();
						Callback::from(move |_| {
							let new_skills: Vec<Skill> = (&**project_skills.clone()).clone().into_iter().filter(|s| s.id != skill.id.clone()).collect();
							project_skills.set(new_skills)
						})
					};
					html! {
						<li>
							<p>{skill.name.to_owned()}</p>
							<button onclick={delete_skill_button}>{"Delete"}</button>
						</li>
					}
				})}
				</ul>
				<label for="new_skill">{"New Skill: "}</label>
				<select name="new_skill" >
					<option disabled=true selected=true>{"Select a skill"}</option>
					{for skills.iter().map(|skill| {
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

			</div>
			<div class="description">{"Created At: "}{project.created_at.to_owned()}</div>
			<div class="description">{"Updated At: "}{project.updated_at.to_owned()}</div>

		</div>
	}
}