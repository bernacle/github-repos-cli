use reqwest::blocking::get;
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
    let repos: Vec<Repo> = get(&url)?.json()?;
    Ok(repos)
}
