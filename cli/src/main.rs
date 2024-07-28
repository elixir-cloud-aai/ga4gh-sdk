use clap::{arg, Command};
use ga4gh_sdk::configuration::Configuration;
use ga4gh_sdk::tes::model::ListTasksParams;
use ga4gh_sdk::transport::Transport;
use std::collections::HashMap;
use std::error::Error;
use ga4gh_sdk::tes::{Task, TES};
use ga4gh_sdk::tes::models::TesTask;
use ga4gh_sdk::test_utils::ensure_funnel_running;
use std::fs;
use std::path::Path;
use ga4gh_sdk::configuration::BasicAuth;
use std::fs::File;
use serde_json::Value;
use std::io::Read;

/// # Examples
///
/// To run the `create` command:
///
/// ```sh
/// cargo run -- tes create '{
///     "name": "Hello world",
///     "inputs": [{
///         "url": "s3://funnel-bucket/hello.txt",
///         "path": "/inputs/hello.txt"
///     }],
///     "outputs": [{
///         "url": "s3://funnel-bucket/output.txt",
///         "path": "/outputs/stdout"
///     }],
///     "executors": [{
///         "image": "alpine",
///         "command": ["cat", "/inputs/hello.txt"],
///         "stdout": "/outputs/stdout"
///     }]
/// }'
/// ```
///
/// Or:
///
/// ```sh
/// cargo run -- tes create './tests/sample.tes'
/// ```
///
/// To run the `list` command:
///
/// ```sh
/// cargo run -- tes list 'name_prefix: None, state: None, tag_key: None, tag_value: None, page_size: None, page_token: None, view: FULL'
/// ```
/// 
/// ASSUME, cqgk5lj93m0311u6p530 is the id of a task created before
/// To run the `get` command:
///
/// ```sh
/// cargo run -- tes get cqgk5lj93m0311u6p530 BASIC
/// ```

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run_cli(Command::new("cli")).await
}
async fn run_cli(cmd: Command<'_>) -> Result<(), Box<dyn Error>> {
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

                .subcommand(
                    Command::new("list")
                        .about("list all tasks")
                        .arg(arg!(<params> "The parameters to get back"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("get")
                        .about("get task data")
                        .arg(arg!(<id> "The id of the task which should be returned"))
                        .arg(arg!(<view> "The view in which the task should be returned"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("status")
                        .about("get status of the task")
                        .arg(arg!(<id> "The id of the task which should be returned"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("cancel")
                        .about("cancel the task")
                        .arg(arg!(<id> "The id of the task which should be cancel"))
                        .arg_required_else_help(true),
                ),
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
                let mut config = load_configuration();
                if config.base_path == "localhost" {
                    let funnel_url = ensure_funnel_running().await;
                    config.set_base_path(&funnel_url);
                }
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
            if let Some(("list", sub)) = sub.subcommand() {          
                let params = sub.value_of("params").unwrap().to_string();
                                
                // Split the params string into key-value pairs and collect into a HashMap for easier access
                let params_map: HashMap<&str, &str> = params.split(',')
                    .filter_map(|s| {
                        let mut parts = s.trim().splitn(2, ':');
                        parts.next().and_then(|key| parts.next().map(|value| (key.trim(), value.trim())))
                    })
                    .collect();
                println!("parameters are: {:?}",params_map);

                // Now, construct ListTasksParams from the parsed values
                let parameters = ListTasksParams {
                    name_prefix: params_map.get("name_prefix").and_then(|&s| if s == "None" { None } else { Some(s.to_string()) }),
                    state: params_map.get("state").and_then(|&s| if s == "None" { None } else { Some(serde_json::from_str(s).expect("Invalid state")) }),
                    tag_key: None, // Example does not cover parsing Vec<String>
                    tag_value: None, // Example does not cover parsing Vec<String>
                    page_size: params_map.get("page_size").and_then(|&s| if s == "None" { None } else { Some(s.parse().expect("Invalid page_size")) }),
                    page_token: params_map.get("page_token").and_then(|&s| if s == "None" { None } else { Some(s.to_string()) }),
                    view: params_map.get("view").and_then(|&s| if s == "None" { None } else { Some(s.to_string()) }),
                };
                println!("parameters are: {:?}",parameters);
                let mut config = load_configuration();
                if config.base_path == "localhost" {
                    let funnel_url = ensure_funnel_running().await;
                    config.set_base_path(&funnel_url);
                }
                match TES::new(&config).await {
                    Ok(tes) => {
                        let task = tes.list_tasks(Some(parameters)).await;
                        println!("{:?}",task);
                    },
                    Err(e) => {
                        println!("Error creating TES instance: {:?}", e);
                        return Err(e);
                    }
                };
            }
            if let Some(("get", sub)) = sub.subcommand() {    
                let id = sub.value_of("id").unwrap();
                let view = sub.value_of("view").unwrap();
                
                let mut config = load_configuration();
                if config.base_path == "localhost" {
                    let funnel_url = ensure_funnel_running().await;
                    config.set_base_path(&funnel_url);
                }

                match TES::new(&config).await {
                    Ok(tes) => {
                        let task = tes.get(view, id).await;
                        println!("{:?}",task);
                    },
                    Err(e) => {
                        println!("Error creating TES instance: {:?}", e);
                        return Err(e);
                    }
                };
            }
            if let Some(("status", sub)) = sub.subcommand() {   
                let id = sub.value_of("id").unwrap().to_string();
                
                let mut config = load_configuration();   
                if config.base_path == "localhost" {
                    let funnel_url = ensure_funnel_running().await;
                    config.set_base_path(&funnel_url);
                }
                let transport = Transport::new(&config);
                let task = Task::new(id, transport);
                match task.status().await {
                    Ok(status) => {
                        println!("The status is: {:?}",status);
                    },
                    Err(e) => {
                        println!("Error creating Task instance: {:?}", e);
                        return Err(e);
                    }
                };
            }
            if let Some(("cancel", sub)) = sub.subcommand() {   
                let id = sub.value_of("id").unwrap().to_string();
                
                let mut config = load_configuration();
                if config.base_path == "localhost" {
                    let funnel_url = ensure_funnel_running().await;
                    config.set_base_path(&funnel_url);
                }
                let transport = Transport::new(&config);
                let task = Task::new(id, transport);
                match task.cancel().await {
                    Ok(output) => {
                        println!("The new value is: {:?}",output);
                    },
                    Err(e) => {
                        println!("Error creating Task instance: {:?}", e);
                        return Err(e);
                    }
                };
            }
        }
        
        _ => {println!("TODO");}
    }
    Ok(())
}

/// Example `config.json` file:
///
/// ```json
/// {
///   "base_path": "http://localhost:8000",
///   "user_agent": "username"
/// }
/// ```

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
    let config_file_path = dirs::home_dir().map(|path| path.join(".config/config.json"));
    if let Some(path) = config_file_path {
        if path.exists() {
            if let Some(path_str) = path.to_str() {
                match read_configuration_from_file(path_str) {
                    Ok(config) => {config},
                    Err(_) => {
                        Configuration::default()
                    },
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
