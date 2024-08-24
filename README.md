# GitHub CLI

Welcome to **GitHub CLI**, a command-line tool for retrieving and managing GitHub repositories. This Rust-powered CLI application lets you quickly fetch and view the latest repositories for a specified GitHub user. Perfect for developers and tech enthusiasts who want a streamlined way to access GitHub repository data right from the terminal!

## Features

- **Fetch Latest Repositories:** Get the latest 10 repositories from a GitHub user with a simple command.
- **View Repository Details:** Display repository names and URLs in a clean, easy-to-read format.
- **Open in Browser:** Select a repository and open it directly in your browser for more details.

## Installation

To get started with GitHub CLI, you'll need to have Rust installed on your machine. If you haven’t installed Rust yet, you can get it from [the official Rust website](https://www.rust-lang.org/).

Clone this repository and build the project using Cargo:

```bash
git clone https://github.com/yourusername/github_cli.git
cd github_cli
cargo build --release
```

## Usage

To use GitHub CLI, run the following command:

```bash
./target/release/github_repos_cli <USERNAME>
```
