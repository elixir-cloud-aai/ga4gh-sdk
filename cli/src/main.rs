use clap::{arg, Command};
use ga4gh_sdk::configuration::Configuration;
use core::task;
use std::error::Error;
use ga4gh_sdk::tes::TES;
use ga4gh_sdk::tes::models::TesTask;
// use std::fs::File;
// use std::io::Write;
// use tempfile::tempdir;


// TO RUN:
// cargo run -- tes create '{
//     "name": "Hello world",
//     "inputs": [{
//         "url": "s3://funnel-bucket/hello.txt",
//         "path": "/inputs/hello.txt"
//     }],
//     "outputs": [{
//         "url": "s3://funnel-bucket/output.txt",
//         "path": "/outputs/stdout"
//     }],
//     "executors": [{
//         "image": "alpine",
//         "command": ["cat", "/inputs/hello.txt"],
//         "stdout": "/outputs/stdout"
//     }]
// }' 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run_cli(Command::new("cli")).await
}
async fn run_cli<'a>(cmd: Command<'a>) -> Result<(), Box<dyn Error>> {
    let cmd = cmd
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
                        // .arg(arg!(--url <URL> "The URL for the task"))
                        .arg_required_else_help(true),
                )
        );

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("tes", sub)) => {
            if let Some(("create", sub)) = sub.subcommand() {
                let task_file = sub.value_of("TASK_FILE").unwrap();
                // let url = sub.value_of("url").unwrap();
                let task_json = task_file.to_string();
                let testask: TesTask = serde_json::from_str(&task_json).expect("JSON was not well-formatted");
                let config = Configuration::default();
                match TES::new(&config).await {
                        Ok(tes) => {
                            let task = tes.create(testask).await;
                            println!("{:?}",task);
                        },
                        Err(e) => {
                            println!("Error creating TES instance: {:?}", e);
                            return Err(e);
                        }
                    };
                }
        }
        _ => {println!("TODO");}
    }
    Ok(())
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_create_task() {
//         // Create a temporary directory to store the task file
//         let temp_dir = tempdir().expect("Failed to create temporary directory");
//         let task_file_path = temp_dir.path().join("task.json");

//         // Create a sample task JSON
//         let task_json = r#"{
//             "name": "Hello world",
//             "inputs": [{
//                 "url": "s3://funnel-bucket/hello.txt",
//                 "path": "/inputs/hello.txt"
//             }],
//             "outputs": [{
//                 "url": "s3://funnel-bucket/output.txt",
//                 "path": "/outputs/stdout"
//             }],
//             "executors": [{
//                 "image": "alpine",
//                 "command": ["cat", "/inputs/hello.txt"],
//                 "stdout": "/outputs/stdout"
//             }]
//             }"#;

//         // Write the task JSON to the temporary file
//         let mut task_file = File::create(&task_file_path).expect("Failed to create task file");
//         task_file
//             .write_all(task_json.as_bytes())
//             .expect("Failed to write task JSON to file");

//         // Call the create function with the temporary file path and a sample URL
//         let url = "http://localhost:8000";
//         let cmd = Command::new("cli")
//             .bin_name("cli")
//             .version("1.0")
//             .about("CLI to manage tasks")
//             .subcommand_required(true)
//             .arg_required_else_help(true)
//             .subcommand(
//                 Command::new("tes")
//                     .about("TES subcommands")
//                     .subcommand_required(true)
//                     .arg_required_else_help(true)
//                     .subcommand(
//                         Command::new("create")
//                             .about("Create a task")
//                             .arg(arg!(<TASK_FILE> "The task file to create"))
//                             .arg(arg!(--url <URL> "The URL for the task").required(true))
//                             .arg_required_else_help(true),
//                     ),
//             );

//         let matches = cmd.clone().get_matches_from(&[
//             "cli",
//             "tes",
//             "create",
//             task_file_path.to_str().unwrap(),
//             "--url",
//             "http://localhost:8000",
//         ]);

//         // Call the run_cli function with the simulated arguments
//         assert!(run_cli(cmd).is_ok());

//         // Additional assertions or verifications can be added here
//     }
// }
