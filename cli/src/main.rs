use ga4gh_sdk::clients::tes::{Task, TES};
use ga4gh_sdk::utils::configuration::Configuration;
use ga4gh_sdk::utils::transport::Transport;
use ga4gh_sdk::utils::extension::InstalledExtension;
use ga4gh_sdk::clients::ServiceType;
use ga4gh_sdk::clients::tes::models::ListTasksParams;
use ga4gh_sdk::clients::tes::models::TesListTasksResponse;
use ga4gh_sdk::clients::tes::models::TesState;
use ga4gh_sdk::clients::tes::models::TesTask;
use clap::{arg, Arg, Command};
use std::path::Path;
use std::error::Error;
use log::{debug, error};
use ga4gh_sdk::utils::expand_path_with_home_dir;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let version = env::var("PACKAGE_VERSION").unwrap_or_else(|_| "unknown".to_string());

    let cmd = Command::new("GA4GH-CLI")
        .bin_name("ga4gh-cli")
        .version(version.as_str())
        .about("GA4GH-CLI is a versatile command-line interface for GA4GH federated cloud environments, \
                built on the GA4GH-SDK Rust library. Designed to simplify interactions with core API services, \
                it currently supports TES, with plans to expand to WES, DRS, TRS, and AAI.\n\
                Contributors are welcome: https://github.com/elixir-cloud-aai/ga4gh-sdk")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .takes_value(true)
                .possible_values(&["trace", "debug", "info", "warn", "error", "off"])
                .default_value("info")
                .help("Sets the level of verbosity"),
        )
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
                        .arg(arg!(-n --name_prefix [NAME_PREFIX] "The name prefix to filter tasks"))
                        .arg(arg!(-s --state [STATE] "The state to filter tasks"))
                        .arg(arg!(-k --tag_key [TAG_KEY] "The tag key to filter tasks"))
                        .arg(arg!(-v --tag_value [TAG_VALUE] "The tag value to filter tasks"))
                        .arg(arg!(-p --page_size [PAGE_SIZE] "The page size for pagination"))
                        .arg(arg!(-t --page_token [PAGE_TOKEN] "The page token for pagination"))
                        .arg(arg!(-w --view [VIEW] "The view for the tasks"))
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
                )
        )
        .subcommand(
            Command::new("extension")
                .about("Extension subcommands")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("list")
                        .about("List all extensions"),
                )
                .subcommand(
                    Command::new("add")
                        .about("Load an extension")
                        .arg(arg!(<file> "The extension file to load"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Unload an extension")
                        .arg(arg!(<name> "The name of the extension to unload"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("enable")
                        .about("Enable an extension")
                        .arg(arg!(<name> "The name of the extension to unload"))
                        .arg_required_else_help(true),
                )
                .subcommand(
                    Command::new("disable")
                        .about("Enable an extension")
                        .arg(arg!(<name> "The name of the extension to unload"))
                        .arg_required_else_help(true),
                )
        );
    
    let matches = cmd.clone().get_matches();

    let log_level = matches.value_of("verbose").unwrap_or("info");
    env::set_var("RUST_LOG", log_level);
    env_logger::init();

    let config_dir_path = expand_path_with_home_dir(".ga4gh");
    if !Path::new(config_dir_path.as_str()).exists() {
        debug!("Creating directory: {}", config_dir_path);
        if let Err(e) = fs::create_dir_all(&config_dir_path) {
            error!("Failed to create directory: {}", e);
        }
    }

    let service_config_path = expand_path_with_home_dir(".ga4gh/config.json");
    let extensions_config_path = expand_path_with_home_dir(".ga4gh/extensions.json");

    match matches.subcommand() {
        Some(("tes", sub)) => {       
            let config = Configuration::from_file(Some(ServiceType::TES), &service_config_path, &extensions_config_path)?;
            let transport = Transport::new(&config)?;
                    
            if let Some(("create", sub)) = sub.subcommand() {
                let task_file = sub.value_of("TASK_FILE")
                    .ok_or_else(|| anyhow::anyhow!("TASK_FILE argument is required"))?;
                let path = Path::new(task_file);
                if !path.exists() {
                    error!("File does not exist: {:?}", path);
                    std::process::exit(1);
                }
                let task_json = match std::fs::read_to_string(path) {
                    Ok(contents) => contents,
                    Err(e) => {
                        error!("Failed to read file: {}", e);
                        task_file.to_string()
                    },
                };

                let testask: TesTask = serde_json::from_str(&task_json)
                    .map_err(|e| format!("Failed to parse GA4GH TES Task JSON: {}", e))?;

                match TES::new(&config).await {
                    Ok(tes) => {
                        let task = tes.create(testask).await?;
                        println!("TASK ID: {}", task.id);
                    },
                    Err(e) => {
                        error!("Error creating TES instance: {:?}", e);
                        return Err(e);
                    }
                };
            }

            if let Some(("list", sub)) = sub.subcommand() {
                let name_prefix = sub.value_of("name_prefix").map(|s| s.to_string());
                let state = sub.value_of("state").map(|s| serde_json::from_str(s).expect("Invalid state"));
                let _tag_key = sub.value_of("tag_key").map(|s| s.to_string());
                let _tag_value = sub.value_of("tag_value").map(|s| s.to_string());
                let page_size = sub.value_of("page_size").map(|s| s.parse().expect("Invalid page_size"));
                let page_token = sub.value_of("page_token").map(|s| s.to_string());
                let view = sub.value_of("view").map(|s| s.to_string());

                let parameters = ListTasksParams {
                    name_prefix,
                    state,
                    tag_key: None, // Example does not cover parsing Vec<String>
                    tag_value: None, // Example does not cover parsing Vec<String>
                    page_size,
                    page_token,
                    view,
                };
                debug!("parameters are: {:?}", parameters);

                match TES::new(&config).await {
                    Ok(tes) => {
                        match tes.list_tasks(Some(parameters)).await {
                            Ok(task_response) => {
                                println!("{}", format_tasks_response(&task_response)); 
                            },
                            Err(e) => {
                                error!("Error listing tasks: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        error!("Error creating TES instance: {:?}", e);
                        return Err(e);
                    }
                };
            }

            if let Some(("get", sub)) = sub.subcommand() {    
                let id = sub.value_of("id").unwrap();
                let view = sub.value_of("view").unwrap();
                let tes = TES::new(&config).await?;
                let task = tes.get(view, id).await?;
                let task_pretty_json = serde_json::to_string_pretty(&task).unwrap();
                println!("{}", task_pretty_json);
            }

            if let Some(("status", sub)) = sub.subcommand() {   
                let id = sub.value_of("id").unwrap().to_string();
                let task = Task::new(id.clone(), transport.clone());
                match task.status().await {
                    Ok(status) => {
                        println!("TASKID: {}", id.clone());
                        println!("STATUS: {:?}", status);
                    },
                    Err(e) => {
                        error!("Error creating Task instance: {:?}", e);
                        return Err(e);
                    }
                };
            }

            if let Some(("cancel", sub)) = sub.subcommand() {   
                let id = sub.value_of("id").unwrap().to_string();
                let task = Task::new(id, transport.clone());
                match task.cancel().await {
                    Ok(output) => {
                        println!("STATUS: {:?}", output);
                    }
                    Err(e) => {
                        error!("Error creating Task instance: {:?}", e);
                        return Err(e);
                    }
                };
            }
        }

        Some(("extension", sub)) => {
            let config = Configuration::from_file(None, &service_config_path, &extensions_config_path)?;

            if let Some(("list", _)) = sub.subcommand() {
                let extensions = config.extensions_manager.get_extensions();
                println!("{}", format_extensions(extensions));
            }

            if let Some(("add", sub)) = sub.subcommand() {
                let file = sub.value_of("file").unwrap();
                config.extensions_manager.add_extension(file).await?;
            }

            if let Some(("remove", sub)) = sub.subcommand() {
                let name = sub.value_of("name").unwrap();
                config.extensions_manager.remove_extension(name)?;
            }

            if let Some(("enable", sub)) = sub.subcommand() {
                let file = sub.value_of("name").unwrap();
                config.extensions_manager.enable_extension(file)?;
            }

            if let Some(("disable", sub)) = sub.subcommand() {
                let file = sub.value_of("name").unwrap();
                config.extensions_manager.disable_extension(file)?;
            }
        }
        
        _ => {
            error!("Error: Unrecognized command or option");
            std::process::exit(1);
        }
    }
    Ok(())
}

// Helper functions

fn tes_state_to_str(state: &Option<TesState>) -> &str {
    match state {
        Some(TesState::Unknown) => "Unknown",
        Some(TesState::Queued) => "Queued",
        Some(TesState::Initializing) => "Initializing",
        Some(TesState::Running) => "Running",
        Some(TesState::Paused) => "Paused",
        Some(TesState::Complete) => "Complete",
        Some(TesState::ExecutorError) => "Executor Error",
        Some(TesState::SystemError) => "System Error",
        Some(TesState::Canceled) => "Canceled",
        Some(TesState::Canceling) => "Canceling",
        Some(TesState::Preempted) => "Preempted",
        None => "None",
    }
}

fn format_task(task: &TesTask) -> String {
    format!(
        "{:<25} {:<15}\n",
        task.id.as_deref().unwrap_or("None"),
        tes_state_to_str(&task.state)
    )
}

fn format_tasks_response(response: &TesListTasksResponse) -> String {
    let mut table = String::new();
    let headers = format!("{:<25} {:<15}\n", "TASK ID", "State");
    table.push_str(&headers);
    for task in &response.tasks {
        table.push_str(&format_task(task));
    }
    table
}

fn format_extensions(extensions: &Vec<InstalledExtension>) -> String {
    let mut table = String::new();
    let headers = format!("{:<25} {:<15}\n", "Extension Name", "Enabled");
    table.push_str(&headers);
    for extension in extensions {
        let row = format!(
            "{:<25} {:<15}\n",
            extension.name.as_str(),
            extension.enabled
        );
        table.push_str(&row);
    }
    table
}