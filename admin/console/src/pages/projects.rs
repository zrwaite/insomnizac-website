use yew::{html, Html, function_component, Properties};
use reqwasm::http::Request;
use yew::{use_effect_with_deps, UseStateHandle};
use yew::prelude::use_state;
use serde::{Deserialize, Serialize};
use log::info;


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Period {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub is_daytime: bool,
    pub temperature: f32,
    pub temperature_unit: String,
    pub wind_speed: String,
    pub wind_direction: String,
    pub icon: String,
    pub short_forecast: String,
    pub detailed_forecast: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Properties2 {
    pub periods: Vec<Period>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Forecast {
    pub properties: Properties2,
}

#[derive(PartialEq, Properties)]
struct PeriodComponentProps {
    pub period: Period,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub github_name: String,
    pub devpost_link: Option<String>,
    pub project_link: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub image: Option<String>,
    pub featured: bool,
    pub skill_ids: Vec<String>,
}

#[function_component(PeriodComponent)]
fn period_component(props: &PeriodComponentProps) -> Html {
    let PeriodComponentProps { period } = props;
    html! {
        <div class="period">
            <div class="name">{period.name.to_owned()}</div>
            <div class="temp">{period.temperature.to_owned()}{period.temperature_unit.to_owned()}</div>
            <div class="forecast">{period.short_forecast.to_owned()}</div>
            <img src={period.icon.to_owned()}/>
        </div>
    }
}


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
            // <button onclick={retry}>{ "Retry" }</button>
            <div>
                {
                    for projects.iter().map(|project| {
                        html! {
                            <div class="project">
                                <div class="name">{project.name.to_owned()}</div>
                                <div class="image"><img src={project.image.to_owned()}/></div>
                                <a class="description" href={project.devpost_link.to_owned()}>{project.devpost_link.to_owned()}</a>
                                <a class="description" href={project.project_link.to_owned()}>{project.project_link.to_owned()}</a>
                                <div class="description">{"Featured: "}{project.featured.to_owned()}</div>
                                <div class="description">{"Skills: "}{project.skill_ids.to_owned()}</div>
                                <div class="description">{"Created At: "}{project.created_at.to_owned()}</div>
                                <div class="description">{"Updated At: "}{project.updated_at.to_owned()}</div>
                            </div>
                        }
                    })
                }
            </div>
        </div>
    }
}