pub mod models;
use crate::configuration::Configuration;
use crate::serviceinfo::models::Service;
use crate::serviceinfo::ServiceInfo;
use crate::tes::models::TesListTasksResponse;
use crate::tes::models::TesState;
use crate::tes::models::TesTask;
use crate::transport::Transport;
use serde_json;
use serde_json::from_str;
use serde_json::json;

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

/// struct for passing parameters to the method [`list_tasks`]
#[derive(Clone, Debug)]
pub struct ListTasksParams {
    /// OPTIONAL. Filter the list to include tasks where the name matches this prefix. If unspecified, no task name filtering is done.
    pub name_prefix: Option<String>,
    /// OPTIONAL. Filter tasks by state. If unspecified, no task state filtering is done.
    pub state: Option<models::TesState>,
    /// OPTIONAL. Provide key tag to filter. The field tag_key is an array of key values, and will be zipped with an optional tag_value array. So the query: ```   ?tag_key=foo1&tag_value=bar1&tag_key=foo2&tag_value=bar2 ``` Should be constructed into the structure { \"foo1\" : \"bar1\", \"foo2\" : \"bar2\"}  ```   ?tag_key=foo1 ``` Should be constructed into the structure {\"foo1\" : \"\"}  If the tag_value is empty, it will be treated as matching any possible value. If a tag value is provided, both the tag's key and value must be exact matches for a task to be returned. Filter                            Tags                          Match? ---------------------------------------------------------------------- {\"foo\": \"bar\"}                    {\"foo\": \"bar\"}                Yes {\"foo\": \"bar\"}                    {\"foo\": \"bat\"}                No {\"foo\": \"\"}                       {\"foo\": \"\"}                   Yes {\"foo\": \"bar\", \"baz\": \"bat\"}      {\"foo\": \"bar\", \"baz\": \"bat\"}  Yes {\"foo\": \"bar\"}                    {\"foo\": \"bar\", \"baz\": \"bat\"}  Yes {\"foo\": \"bar\", \"baz\": \"bat\"}      {\"foo\": \"bar\"}                No {\"foo\": \"\"}                       {\"foo\": \"bar\"}                Yes {\"foo\": \"\"}                       {}                            No
    pub tag_key: Option<Vec<String>>,
    /// OPTIONAL. The companion value field for tag_key
    pub tag_value: Option<Vec<String>>,
    /// Optional number of tasks to return in one page. Must be less than 2048. Defaults to 256.
    pub page_size: Option<i32>,
    /// OPTIONAL. Page token is used to retrieve the next page of results. If unspecified, returns the first page of results. The value can be found in the `next_page_token` field of the last returned result of ListTasks
    pub page_token: Option<String>,
    /// OPTIONAL. Affects the fields included in the returned Task messages.  `MINIMAL`: Task message will include ONLY the fields: - `tesTask.Id` - `tesTask.State`  `BASIC`: Task message will include all fields EXCEPT: - `tesTask.ExecutorLog.stdout` - `tesTask.ExecutorLog.stderr` - `tesInput.content` - `tesTaskLog.system_logs`  `FULL`: Task message includes all fields.
    pub view: Option<String>,
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
            Err(e) => Err(e),
        }
    }

    pub async fn cancel(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let id = &self.id;
        let id = &urlencode(id);
        let url = format!("/tasks/{}:cancel", id);
        // let url= &urlencode(url);
        // println!("{:?}",url);
        let response = self.transport.post(&url, None).await;
        // println!("the response is: {:?}",response);
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
// *** see question above

impl TES {
    pub async fn new(config: &Configuration) -> Result<Self, Box<dyn std::error::Error>> {
        let transport = Transport::new(config);
        let service_info = ServiceInfo::new(config)?;

        let resp = service_info.get().await;

        // println!("artifact: {}",resp.clone().unwrap().r#type.artifact);
        let instance = TES {
            config: config.clone(),
            transport,
            service: resp,
        };

        if instance.check() {
            Ok(instance)
        } else {
            Err("The endpoint is not an instance of TES".into())
        }
    }

    fn check(&self) -> bool {
        let resp = &self.service;
        match resp.as_ref() {
            Ok(service) => service.r#type.artifact == "tes",
            Err(_) => false, // or handle the error as appropriate
        }
    }

    pub async fn create(
        &self,
        task: TesTask, /*, params: models::TesTask*/
    ) -> Result<Task, Box<dyn std::error::Error>> {
        // First, check if the service is of TES class
        if !self.check() {
            // If check fails, log an error and return an Err immediately
            log::error!("Service check failed");
            return Err("Service check failed".into());
        }
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
        let params_value = params.map(|p| {
            let mut map = serde_json::Map::new();
            
            if let Some(name_prefix) = p.name_prefix {
                map.insert("name_prefix".to_string(), json!(name_prefix));
            }
            if let Some(state) = p.state {
                map.insert("state".to_string(), json!(state));
            }
            if let Some(tag_key) = p.tag_key {
                map.insert("tag_key".to_string(), json!(tag_key));
            }
            if let Some(tag_value) = p.tag_value {
                map.insert("tag_value".to_string(), json!(tag_value));
            }
            if let Some(page_size) = p.page_size {
                map.insert("page_size".to_string(), json!(page_size));
            }
            if let Some(page_token) = p.page_token {
                map.insert("page_token".to_string(), json!(page_token));
            }
            if let Some(view) = p.view {
                map.insert("view".to_string(), json!(view));
            }

            json!(map)
        });
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
            Err(e) => Err(e),
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
        let file_path = std::env::var("TASK_FILE_PATH").unwrap_or_else(|_| "./lib/sample/grape.tes".to_string());
        let task_json = std::fs::read_to_string(file_path).expect("Unable to read file");
        let task: TesTask = serde_json::from_str(&task_json).expect("JSON was not well-formatted");

        let task = tes.create(task).await?;
        Ok((task, tes))
    }

    #[tokio::test]
    async fn test_task_create() {
        setup();
        let (task, _tes) = create_task().await.expect("Failed to create task");
        assert!(!task.id.is_empty(), "Task ID should not be empty"); // doube check if it's a correct assertion
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
        assert!(!task.id.clone().is_empty(), "Task ID should not be empty"); // double check if it's a correct assertion

        let cancel = task.cancel().await;
        assert!(cancel.is_ok());
    }

    #[tokio::test]
    async fn test_list_task() {
        setup();

        let (task, tes) = &create_task().await.expect("Failed to create task");
        assert!(!task.id.clone().is_empty(), "Task ID should not be empty"); // doube check if it's a correct assertion

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
