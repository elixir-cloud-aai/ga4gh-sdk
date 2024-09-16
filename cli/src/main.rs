use clap::{arg, Command};
use ga4gh_sdk::clients::tes::models::ListTasksParams;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use ga4gh_sdk::clients::tes::models::TesTask;
use ga4gh_sdk::clients::tes::{Task, TES};
use ga4gh_sdk::utils::configuration::Configuration;
use ga4gh_sdk::utils::test_utils::ensure_funnel_running;
use ga4gh_sdk::utils::transport::Transport;
use ga4gh_sdk::utils::configuration::BasicAuth;
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
/// OR
/// Parameters with None values can be avoided, like:
/// ```sh
/// ga4gh-cli tes list 'view:FULL'
/// ```
/// 
/// ASSUME, crjpvmb93m0bq6ssgqn0 is the id of a task created before
/// To run the `get` command:
///
/// ```sh
/// cargo run -- tes get crjpvmb93m0bq6ssgqn0 BASIC
/// ```
/// /// To run the `status` command:
///
/// ```sh
/// ga4gh-cli tes status crjpvmb93m0bq6ssgqn0      
/// ```
///
///
/// To run the `cancel` command:
///
/// ```sh
/// ga4gh-cli tes cancel crjpvmb93m0bq6ssgqn0      
/// ```

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cmd = Command::new("cli")
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
                let task_file = sub.value_of("TASK_FILE")
                    .ok_or_else(|| anyhow::anyhow!("TASK_FILE argument is required"))?;
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
                let testask: TesTask = serde_json::from_str(&task_json)
                    .map_err(|e| format!("Failed to parse JSON: {}", e))?;
                let mut config = load_configuration();
                if config.base_path.as_str() == "localhost" {
                    let funnel_url = ensure_funnel_running().await;
                    let funnel_url = url::Url::parse(&funnel_url).expect("Invalid URL");
                    config.set_base_path(funnel_url);
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
                let params_map: HashMap<String, String> = params
                    .split(',')
                    .filter_map(|s| {
                        let mut parts = s.trim().splitn(2, ':');
                        Some((parts.next()?.to_string(), parts.next()?.to_string()))
                    })
                    .collect();
                println!("parameters are: {:?}",params_map);

                // Now, construct ListTasksParams from the parsed values
                let parameters = ListTasksParams {
                    name_prefix: params_map.get("name_prefix").and_then(|s| if s == "None" { None } else { Some(s.to_string()) }),
                    state: params_map.get("state").and_then(|s| if s == "None" { None } else { Some(serde_json::from_str(s).expect("Invalid state")) }),
                    tag_key: None, // Example does not cover parsing Vec<String>
                    tag_value: None, // Example does not cover parsing Vec<String>
                    page_size: params_map.get("page_size").and_then(|s| if s == "None" { None } else { Some(s.parse().expect("Invalid page_size")) }),
                    page_token: params_map.get("page_token").and_then(|s| if s == "None" { None } else { Some(s.to_string()) }),
                    view: params_map.get("view").and_then(|s| if s == "None" { None } else { Some(s.to_string()) }),
                };
                println!("parameters are: {:?}",parameters);
                let mut config = load_configuration();
                if config.base_path.as_str() == "localhost" {
                    let funnel_url = ensure_funnel_running().await;
                    let funnel_url = url::Url::parse(&funnel_url).expect("Invalid URL");
                    config.set_base_path(funnel_url);
                }
                match TES::new(&config).await {
                    Ok(tes) => {
                        let task = tes.list_tasks(Some(parameters)).await;
                        println!("{:?}",task);
                    },
                    Err(e) => {
                        eprintln!("Error creating TES instance: {:?}", e);
                        return Err(e);
                    }
                };
            }
            if let Some(("get", sub)) = sub.subcommand() {    
                let id = sub.value_of("id").unwrap();
                let view = sub.value_of("view").unwrap();
                
                let mut config = load_configuration();
                if config.base_path.as_str() == "localhost" {
                    let funnel_url = ensure_funnel_running().await;
                    let funnel_url = url::Url::parse(&funnel_url).expect("Invalid URL");
                    config.set_base_path(funnel_url);
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
                if config.base_path.as_str() == "localhost" {
                    let funnel_url = ensure_funnel_running().await;
                    let funnel_url = url::Url::parse(&funnel_url).expect("Invalid URL");
                    config.set_base_path(funnel_url);
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
                if config.base_path.as_str() == "localhost" {
                    let funnel_url = ensure_funnel_running().await;
                    let funnel_url = url::Url::parse(&funnel_url).expect("Invalid URL");
                    config.set_base_path(funnel_url);
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
        
        _ => {
            eprintln!("Error: Unrecognized command or option");
            std::process::exit(1);
        }
    }
    Ok(())
}

/// Loads the configuration from a JSON file.
///
/// # Example `config.json`
///
/// ```json
/// {
///   "base_path": "http://localhost:8000",
///   "user_agent": "Some(User)",
///   "basic_auth": {
///         "username": "your_username",
///         "password": "your_password"
///     },
///   "oauth_access_token": "your_oauth_access_token"
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the configuration file is missing or malformed.


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
    let base_path = url::Url::parse(&base_path).expect("Invalid URL");
    let config = Configuration::new(base_path)
        .with_user_agent(user_agent.expect("Invalid user agent"))
        .with_basic_auth(basic_auth.expect("Invalid basic_auth"))
        .with_oauth_access_token(oauth_access_token.expect("Invalid oauth access token"));
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
