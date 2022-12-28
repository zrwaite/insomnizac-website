use log::info;
use reqwasm::http::Request;
use serde::Deserialize;

use crate::models::RailsError;

pub enum HttpMethod {
	GET,
	POST,
	PUT,
	DELETE,
}

pub enum HttpResponse<T> {
	Success(T),
	Error(RailsError),
	Unknown(String),
}

pub async fn http_request<T: for<'a> Deserialize<'a>>(endpoint: String, method: HttpMethod, body: Option<String>) -> HttpResponse<T> {
	let mut request = match method {
		HttpMethod::GET => Request::get(&endpoint),
		HttpMethod::POST => Request::post(&endpoint),
		HttpMethod::PUT => Request::put(&endpoint),
		HttpMethod::DELETE => Request::delete(&endpoint),
	};
	// update_request.header("Content-Type", "application/json");
	request = request.body(body);

	let request = request.send().await;

	return match request {
		Ok(response) => {
			let text = response.text().await.unwrap();
			let good_response: Result<T, _> = serde_json::from_str(&text.clone());
			let error_response: Result<RailsError, _> = serde_json::from_str(&text.clone());
			match good_response {
				Ok(p) => HttpResponse::Success(p),
				Err(_) => match error_response {
					Ok(e) => HttpResponse::Error(e),
					Err(e) => HttpResponse::Unknown(e.to_string())
				}
			}
		}
		Err(e) => HttpResponse::Unknown(e.to_string())
	}
}