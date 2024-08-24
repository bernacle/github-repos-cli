use crate::github::Repo;
use std::io::{self, Write};

pub fn display_repos(repos: &[Repo]) {
    for (i, repo) in repos.iter().enumerate() {
        println!("{}: {}", i + 1, repo.name);
    }
}

pub fn prompt_user_choice() -> usize {
    print!("Select a repository (1-10): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    choice.trim().parse::<usize>().unwrap_or(0)
}

pub fn open_repo_in_browser(repo: &Repo) {
    open::that(&repo.html_url).expect("Failed to open browser")
}
