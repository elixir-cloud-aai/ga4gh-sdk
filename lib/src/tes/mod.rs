pub mod models;
pub mod model;
use crate::configuration::Configuration;
use crate::serviceinfo::models::Service;
use crate::serviceinfo::ServiceInfo;
use crate::tes::models::TesListTasksResponse;
use crate::tes::models::TesState;
use crate::tes::models::TesTask;
use crate::transport::Transport;
use crate::tes::model::ListTasksParams;
use serde_json;
use serde_json::from_str;
use serde_json::json;
use serde::Serialize;
use serde_json::Value;

fn serialize_to_json<T: Serialize>(item: T) -> Value {
    serde_json::to_value(&item).unwrap()
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

#[derive(Debug)]
pub struct Task {
    id: String,
    transport: Transport,
}

impl Task {
    pub fn new(id: String, transport: Transport) -> Self {
        Task { id, transport }
    }

    pub async fn status(&self) -> Result<TesState, Box<dyn std::error::Error>> {
        let task_id = &self.id;
        let view = "FULL";
        let url = format!("/tasks/{}?view={}", task_id, view);
        // let params = [("view", view)];
        // let params_value = serde_json::json!(params);
        // let response = self.transport.get(&url, Some(params_value)).await;
        let response = self.transport.get(&url, None).await;
        match response {
            Ok(resp_str) => {
                let task: TesTask = from_str(&resp_str)?;
                Ok(task.state.unwrap())
            }
            Err(e) => {
                let err_msg = format!("HTTP request failed: {}", e);
                eprintln!("{}", err_msg);
                Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, err_msg)))
            }
        }
    }

    pub async fn cancel(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let id = &self.id;
        let id = &urlencode(id);
        let url = format!("/tasks/{}:cancel", id);
        let response = self.transport.post(&url, None).await;
        match response {
	    Ok(resp_str) => {
	        let parsed_json = serde_json::from_str::<serde_json::Value>(&resp_str);
	        match parsed_json {
	            Ok(json) => Ok(json),
	            Err(e) => Err(format!("Failed to parse JSON: {}", e).into()),
	        }
	    }
	    Err(e) => Err(format!("HTTP request failed: {}", e).into()),
	}
    }
}
#[derive(Debug)]
pub struct TES {
    #[allow(dead_code)]
    config: Configuration, // not used yet
    service: Result<Service, Box<dyn std::error::Error>>,
    transport: Transport,
}

impl TES {
    pub async fn new(config: &Configuration) -> Result<Self, Box<dyn std::error::Error>> {
        let transport = Transport::new(config);
        let service_info = ServiceInfo::new(config)?;

        let resp = service_info.get().await;

        let instance = TES {
            config: config.clone(),
            transport,
            service: resp,
        };

        instance.check()?; // Propagate the error if check() fails
        Ok(instance)
    }

    fn check(&self) -> Result<(), String> {
        let resp = &self.service;
        match resp.as_ref() {
            Ok(service) if service.r#type.artifact == "tes" => Ok(()),
            Ok(_) => Err("The endpoint is not an instance of TES".into()),
            Err(_) => Err("Error accessing the service".into()),
        }
    }

    pub async fn create(
        &self,
        task: TesTask, /*, params: models::TesTask*/
    ) -> Result<Task, Box<dyn std::error::Error>> {
        // First, check if the service is of TES class
        self.check().map_err(|e| {
            log::error!("Service check failed: {}", e);
            e
        })?;
        // todo: version in url based on serviceinfo or user config
        let response = self
            .transport
            .post("/ga4gh/tes/v1/tasks", Some(json!(task)))
            .await;
        match response {
            Ok(response_body) => {
                let v: serde_json::Value = serde_json::from_str(&response_body)?;

                // Access the `id` field
                let task_id = v
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_string();

                let task = Task {
                    id: task_id,
                    transport: self.transport.clone(),
                };
                Ok(task)
            }
            Err(e) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to post task: {}", e),
            ))),
        }
    }

    pub async fn get(&self, view: &str, id: &str) -> Result<TesTask, Box<dyn std::error::Error>> {
        let task_id = id;
        let url = format!("/tasks/{}?view={}", task_id, view);
        // let params = [("view", view)];
        // let params_value = serde_json::json!(params);
        // let response = self.transport.get(&url, Some(params_value)).await;
        let response = self.transport.get(&url, None).await;
        match response {
            Ok(resp_str) => {
                let task: TesTask = from_str(&resp_str)?;
                Ok(task)
            }
            Err(e) => Err(e),
        }
    }
    pub async fn list_tasks(
        &self,
        params: Option<ListTasksParams>,
    ) -> Result<TesListTasksResponse, Box<dyn std::error::Error>> {
        let params_value = params.map(serialize_to_json);

        // println!("{:?}",params_value);
        // Make the request with or without parameters based on the presence of params
        let response = if let Some(params_value) = params_value {
            self.transport.get("/tasks", Some(params_value)).await
        } else {
            self.transport.get("/tasks", None).await
        };

        match response {
            Ok(resp_str) => {
                let task: TesListTasksResponse = from_str(&resp_str)?;
                Ok(task)
            }
            Err(e) => {
                eprintln!("HTTP request failed: {:?}", e);
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("HTTP request failed: {:?}", e),
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::Configuration;
    use crate::tes::models::TesTask;
    use crate::tes::ListTasksParams;
    use crate::tes::Task;
    use crate::tes::TesState;
    use crate::tes::TES;
    use crate::test_utils::{ensure_funnel_running, setup};
    // use crate::tes::models::TesCreateTaskResponse;

    async fn create_task() -> Result<(Task, TES), Box<dyn std::error::Error>> {
        // setup(); – should be run once in the test function
        let mut config = Configuration::default();
        let funnel_url = ensure_funnel_running().await;
        config.set_base_path(&funnel_url);
        let tes = match TES::new(&config).await {
            Ok(tes) => tes,
            Err(e) => {
                println!("Error creating TES instance: {:?}", e);
                return Err(e);
            }
        };
        let file_path = "./tests/sample.tes".to_string();
        let task_json = std::fs::read_to_string(file_path).expect("Unable to read file");
        let task: TesTask = serde_json::from_str(&task_json).expect("JSON was not well-formatted");

        let task = tes.create(task).await?;
        Ok((task, tes))
    }

    #[tokio::test]
    async fn test_task_create() {
        setup();
        let (task, _tes) = create_task().await.expect("Failed to create task");
        assert!(!task.id.is_empty(), "Task ID should not be empty"); // double check if it's a correct assertion
    }

    #[tokio::test]
    async fn test_task_status() {
        setup();

        let (task, _tes) = create_task().await.expect("Failed to create task");
        assert!(!task.id.is_empty(), "Task ID should not be empty");

        let status = task.status().await;
        match status {
            Ok(state) => {
                assert!(
                    matches!(state, TesState::Initializing | TesState::Queued | TesState::Running),
                    "Unexpected state: {:?}",
                    state
                );
            }
            Err(err) => {
                panic!("Task status returned an error: {:?}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_cancel_task() {
        setup();

        let (task, _tes) = &create_task().await.expect("Failed to create task");
        assert!(!task.id.is_empty(), "Task ID should not be empty"); // double check if it's a correct assertion

        let cancel = task.cancel().await;
        assert!(cancel.is_ok());
    }

    #[tokio::test]
    async fn test_list_task() {
        setup();

        let (task, tes) = &create_task().await.expect("Failed to create task");
        assert!(!task.id.is_empty(), "Task ID should not be empty"); // double check if it's a correct assertion

        let params: ListTasksParams = ListTasksParams {
            name_prefix: None,
            state: None,
            tag_key: None,
            tag_value: None,
            page_size: None,
            page_token: None,
            view: Some("BASIC".to_string()),
        };

        let list = tes.list_tasks(Some(params)).await;
        assert!(list.is_ok());
        println!("{:?}", list);
    }
}