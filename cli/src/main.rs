use clap::{arg, Command};
use std::error::Error;
use crate::tes; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new("cli")
        .bin_name("cli")
        .version("1.0")
        .about("CLI to manage tasks")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("tes")
                .about("TES subcommands")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a task")
                        .arg(arg!(<TASK_FILE> "The task file to create"))
                        .arg(arg!(--url <URL> "The URL for the task").required(true))
                        .arg_required_else_help(true),
                )
        );

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("tes", sub)) => {
            match sub.subcommand() {
                Some(("create", sub)) => {
                    let task_file = sub.value_of("TASK_FILE").unwrap();
                    let url = sub.value_of("url").unwrap();
                    tes::create(task_file, url).await?;
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

// lib/src/tes/mod.rs
pub async fn create(task_file: &str, url: &str) -> Result<(), Box<dyn Error>> {
    // Your implementation here
    println!("Creating task with file: {} and URL: {}", task_file, url);
    Ok(())
}
