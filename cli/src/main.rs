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
/// 
/// To run the `status` command:
///
/// ```sh
/// cargo run -- tes status cqgk5lj93m0311u6p530      
/// ```
/// 
/// 
/// To run the `cancel` command:
///
/// ```sh
/// cargo run -- tes cancel cqgk5lj93m0311u6p530      
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
                let testask: TesTask = serde_json::from_str(&task_json)
                    .map_err(|e| format!("Failed to parse JSON: {}", e))?;
                let mut config = Configuration::default();
                // let mut config = load_configuration();
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
            if let Some(("list", sub)) = sub.subcommand() {               
                let mut config = Configuration::default();
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
                    name_prefix: params_map.get("name_prefix").and_then(|&s| if s == "None" { None } else { Some(s.to_string()) }),
                    state: params_map.get("state").and_then(|&s| if s == "None" { None } else { Some(serde_json::from_str(&s).expect("Invalid state")) }),
                    tag_key: None, // Example does not cover parsing Vec<String>
                    tag_value: None, // Example does not cover parsing Vec<String>
                    page_size: params_map.get("page_size").and_then(|&s| if s == "None" { None } else { Some(s.parse().expect("Invalid page_size")) }),
                    page_token: params_map.get("page_token").and_then(|&s| if s == "None" { None } else { Some(s.to_string()) }),
                    view: params_map.get("view").and_then(|&s| if s == "None" { None } else { Some(s.to_string()) }),
                };
                println!("parameters are: {:?}",parameters);
                // let mut config = load_configuration();
                let funnel_url = ensure_funnel_running().await;
                config.set_base_path(&funnel_url);
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
                let mut config = Configuration::default();
                let id = sub.value_of("id").unwrap();
                let view = sub.value_of("view").unwrap();
                
                // let mut config = load_configuration();
                let funnel_url = ensure_funnel_running().await;
                config.set_base_path(&funnel_url);
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
                let mut config = Configuration::default();
                let id = sub.value_of("id").unwrap().to_string();
                
                // let mut config = load_configuration();
                let funnel_url = ensure_funnel_running().await;
                config.set_base_path(&funnel_url);
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
                let mut config = Configuration::default();
                let id = sub.value_of("id").unwrap().to_string();
                
                // let mut config = load_configuration();
                let funnel_url = ensure_funnel_running().await;
                config.set_base_path(&funnel_url);
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
        }
    }
    Ok(())
}