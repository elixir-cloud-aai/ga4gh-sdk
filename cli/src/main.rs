use clap::{arg, Command};
use ga4gh_sdk::configuration::Configuration;
// use core::task;
use std::error::Error;
use ga4gh_sdk::tes::TES;
use ga4gh_sdk::tes::models::TesTask;
use ga4gh_sdk::configuration::BasicAuth;
use ga4gh_sdk::test_utils::ensure_funnel_running;
use std::fs::File;
use serde_json::Value;
use std::io::Read;
use std::fs;
use std::path::Path;

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

// OR 
// cargo run -- tes create '../tests/sample.tes' 



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
                let path = Path::new(task_file);
                if !path.exists() {
                    eprintln!("File does not exist: {:?}", path);
                }
                let task_json = match fs::read_to_string(path) {
                    Ok(contents) => contents,
                    Err(e) => {
                        eprintln!("Failed to read file: {}", e);
                        task_file.to_string()
                    },
                };
                let testask: TesTask = serde_json::from_str(&task_json).expect("JSON was not well-formatted");
                // let mut config = Configuration::default();
                let mut config = load_configuration();
                let funnel_url = ensure_funnel_running().await;
                config.set_base_path(&funnel_url);
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
fn read_configuration_from_file(file_path: &str) -> Result<Configuration, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let json_value: Value = serde_json::from_str(&contents)?;

    let base_path = json_value["base_path"].as_str().unwrap_or_default().to_string();
    let user_agent = json_value["user_agent"].as_str().map(|s| s.to_string());
    let basic_auth = json_value["basic_auth"].as_object().map(|auth| BasicAuth {
            username: auth["username"].as_str().unwrap_or_default().to_string(),
            password: Some(auth["password"].as_str().unwrap_or_default().to_string()),
        });
    let oauth_access_token = json_value["oauth_access_token"].as_str().map(|s| s.to_string());

    let config = Configuration::new(base_path, user_agent, basic_auth, oauth_access_token);
    Ok(config)
}

fn load_configuration() -> Configuration {
    let config_file_path = dirs::home_dir().map(|path| path.join(".config"));
    if let Some(path) = config_file_path {
        if path.exists() {
            if let Some(path_str) = path.to_str() {
                match read_configuration_from_file(path_str) {
                    Ok(config) => {config},
                    Err(_) => {Configuration::default()},
                }
            } else {
                Configuration::default()
            }
            
        } else {
            Configuration::default()
        }
    } else {
        Configuration::default()
    }
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
