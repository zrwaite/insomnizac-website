use yew::{html, Html, function_component, Properties};
use reqwasm::http::Request;
use yew::{use_effect_with_deps, UseStateHandle};
use yew::prelude::use_state;
use serde::{Deserialize, Serialize};
use log::info;
// use sass_rs::{compile_string};

use crate::models::Project;
use crate::components::ProjectPanel;


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
        <>  
            <head>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
            </head>
            <body>

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
            </body>

        </>
    }
}