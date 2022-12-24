use yew::{function_component, Html, Properties, html};

use crate::models::Project;

#[derive(PartialEq, Properties)]
pub struct ProjectPanelProps {
    pub project: Project
}


#[function_component(ProjectPanel)]
pub fn project_panel(props: &ProjectPanelProps) -> Html {
    let ProjectPanelProps { project } = props;
	html! {
		<div class="project">
			<div class="header">
				
				<h3>{project.name.to_owned()}</h3>
			</div>
			<div class="image"><img src={project.image.to_owned()}/></div>
			<a class="description" href={project.devpost_link.to_owned()}>{project.devpost_link.to_owned()}</a>
			<a class="description" href={project.project_link.to_owned()}>{project.project_link.to_owned()}</a>
			<div class="description">{"Featured: "}{project.featured.to_owned()}</div>
			<div class="description">
				<h4>{"Skills: "}</h4>
				<ul>
					{for project.skills.iter().map(|skill| {
						html! {
							<li>{skill.name.to_owned()}</li>
						}})
					}
				</ul>
			</div>
			<div class="description">{"Created At: "}{project.created_at.to_owned()}</div>
			<div class="description">{"Updated At: "}{project.updated_at.to_owned()}</div>
		</div>
	}
}