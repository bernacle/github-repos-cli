use clap::Parser;

#[derive(Parser)]
#[command(name = "Github CLI")]
#[command(about = "Retrieve latest Github repos")]
pub struct Cli {
    pub username: String,
}
