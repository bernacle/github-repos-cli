use reqwest::header::{HeaderMap, USER_AGENT};
use reqwest::StatusCode;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub name: String,
    pub html_url: String,
}

pub fn fetch_repos(user: &str) -> Result<Vec<Repo>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/users/{}/repos?sort=updated&per_page=10",
        user
    );
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "my-rust-app/1.0".parse()?);

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let response = client.get(&url).send()?;

    let status = response.status();
    let body = response.text()?;

    if status != StatusCode::OK {
        return Err(format!(
            "Error: Received status code {}. Response body: {}. Request URL: {}",
            status, body, url
        )
        .into());
    }

    let repos: Vec<Repo> = serde_json::from_str(&body)?;
    Ok(repos)
}
