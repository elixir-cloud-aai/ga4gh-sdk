pub mod models;
use crate::configuration::Configuration;
use crate::serviceinfo::models::Service;
use crate::serviceinfo::ServiceInfo;
use crate::tes::models::TesState;
use crate::tes::models::TesTask;
use crate::transport::Transport;
use serde_json;
use serde_json::json;
use serde_json::from_str;


// ***
// should TES.create return Task? which in turn can do status() and other existing-task-related stuff
// instead of TES.status(task_id) we could do task.status()
#[derive(Serialize, Deserialize)]
pub struct Task {
    id: String,
}

impl Task {
    pub fn new(id: String) -> Self {
        Task {
            id,
        }
    }

    pub async fn status(&self, tes: &TES) -> Result<TesState, Box<dyn std::error::Error>> {
        let task_id=&self.id;
        // let config = Configuration::default();
        // let tes=TES::new(config).await;
        tes.status(&task_id.clone(), "FULL").await
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
        let service_info = ServiceInfo::new(config).unwrap();

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
        return resp.as_ref().unwrap().r#type.artifact == "tes";
        // true
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
            .post("/ga4gh/tes/v1/tasks", json!(task))
            .await;
        match response {
            Ok(response_body) => {
                match serde_json::from_str::<Task>(&response_body) {
                    Ok(tes_create_task_response) => Ok(tes_create_task_response),
                    Err(e) => {
                        log::error!("Failed to deserialize response: {}", e);
                        Err("Failed to deserialize response".into())
                    }
                }
            }
            Err(e) => {
                log::error!("Error: {}", e);
                Err(e)
            }
        }
    }

    // pub async fn status(&self, task: &TesCreateTaskResponse) -> Result<TesState, Box<dyn std::error::Error>> {
    pub async fn status(
        &self,
        task_id: &str,
        view: &str,// 'view' controls the level of detail in the response. Expected values: "MINIMAL","BASIC" or "FULL".
    ) -> Result<TesState, Box<dyn std::error::Error>> {
        // ?? move to Task::status()
        // todo: version in url based on serviceinfo or user config
        let url = format!("/tasks/{}?view={}", task_id, view);
        // let params = [("view", view)];
        // let params_value = serde_json::json!(params);
        // println!("{:?}", &self);
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

    // TODO: pub fn list()
}

#[cfg(test)]
mod tests {
    use crate::configuration::Configuration;
    use crate::tes::Task;
    use crate::tes::models::TesTask;
    use crate::tes::TES;
    use crate::test_utils::{ensure_funnel_running, setup};
    use crate::tes::TesState;
    // use crate::test_utils::{ensure_funnel_running, setup, FUNNEL_PORT};
    // use crate::tes::models::TesCreateTaskResponse;

    async fn create_task() -> Result<String, Box<dyn std::error::Error>> {
        setup();
        let mut config = Configuration::default();
        let funnel_url = ensure_funnel_running().await;
        config.set_base_path(&funnel_url);
        let tes = TES::new(&config).await;

        let task_json =
            std::fs::read_to_string("./lib/sample/grape.tes").expect("Unable to read file");
        let task: TesTask = serde_json::from_str(&task_json).expect("JSON was not well-formatted");

        let task = tes?.create(task).await?;
        Ok(task.id)
    }

    #[tokio::test]
    async fn test_task_create() {
        setup();
        ensure_funnel_running().await;

        let task = create_task().await.expect("Failed to create task");
        assert!(!task.is_empty(), "Task ID should not be empty"); // doube check if it's a correct assertion
    }

    #[tokio::test]
    async fn test_task_status() {
        setup();

        let taskid = &create_task().await.expect("Failed to create task");
        assert!(!taskid.clone().is_empty(), "Task ID should not be empty"); // doube check if it's a correct assertion

        let task=Task::new(taskid.clone());

        let mut config = Configuration::default();
        let funnel_url = ensure_funnel_running().await;
        config.set_base_path(&funnel_url);
        match TES::new(&config).await {
            Ok(tes) => {
                let status = task.status(&tes).await;
                println!("Task: {:?}", status);
                // Adding an assertion for the Ok variant
                match status {
                    Ok(state) => {
                        match state {
                            TesState::Initializing | TesState::Queued => {
                                // Assertion passes if state is Initializing or Queued
                            }
                            _ => {
                                panic!("Unexpected state: {:?}", state);
                            }
                        }

                    }
                    Err(err) => {
                        panic!("Task status returned an error: {:?}", err);
                    }
                }

            },
            Err(e) => {
                // Handle the error e
                println!("Error creating TES instance: {:?}", e);
            }
}
        // Now use task to get the task status...
        // todo: assert_eq!(task.status().await, which status?);
    }
}
