mod cli;
mod github;
mod ui;

use clap::Parser;
use cli::Cli;
use github::fetch_repos;
use ui::{display_repos, open_repo_in_browser, prompt_user_choice};

fn main() {
    let args = Cli::parse();
    match fetch_repos(&args.username) {
        Ok(repos) => {
            if repos.is_empty() {
                println!("No repos found for the user: {}", args.username)
            }

            display_repos(&repos);
            let choice = prompt_user_choice();

            if choice > 0 && choice <= repos.len() {
                open_repo_in_browser(&repos[choice - 1]);
            } else {
                println!("Invalid choice")
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}
