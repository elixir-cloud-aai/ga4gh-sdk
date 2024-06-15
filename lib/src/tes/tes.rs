use reqwest;

use crate::{tes::ResponseContent, tes::models};
use super::{Error, configuration};
use crate::service::Service;
use serde_json::json;
use crate::tes::models::TesCreateTaskResponse;


// Defining service class
pub struct Tes {
    service: Service,
}

/// struct for passing parameters to the method [`create_task`]
#[derive(Clone, Debug)]
pub struct CreateTaskParams {
    pub body: models::TesTask
}

/// struct for typed errors of method [`create_task`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTaskError {
    UnknownValue(serde_json::Value),
}

impl Tes {
    pub fn new(service: Service) -> Self {
        Tes { service }
    }

    /// Create a new task. The user provides a Task document, which the server uses as a basis and adds additional fields.
    // pub async fn create_task(&self, configuration: &configuration::Configuration, params: CreateTaskParams) -> Result<models::TesCreateTaskResponse, Error<CreateTaskError>> {
    pub async fn create_task(&self, configuration: &configuration::Configuration, params: CreateTaskParams) {
        let local_var_configuration = configuration;

        // unbox the parameters
        let body = params.body;
        
        let local_var_uri_str = format!("{}/tasks", local_var_configuration.base_path);
        let endpoint = local_var_uri_str.as_str();
        let response = self.service.request(reqwest::Method::POST, endpoint, Some(serde_json::to_value(body).unwrap()), None).await;


        // if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        //     serde_json::from_str(&local_var_content).map_err(Error::from)
        // } else {
        //     let local_var_entity: Option<CreateTaskError> = serde_json::from_str(&local_var_content).ok();
        //     let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        //     Err(Error::ResponseError(local_var_error))
        // }


        
        match response {
            Ok(content) => {
                // Handle the successful response
                println!("Success: {}", content);
                // You can also process the content as needed
            },
            Err(e) => {
                // Handle the error
                eprintln!("Error: {}", e);
                // You can also process the error as needed
            },
        }

    }

 
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tes::configuration::Configuration;
    use crate::tes::tes::reqwest::Client;
    use crate::tes::models::TesTask;

    #[tokio::test]
    async fn test_create_task() {
        let client_tes = Client::new();

        // Define the configuration
        let config = Configuration {
            base_path: "http://localhost:8000".to_string(),
            user_agent: None,
            client: client_tes.clone(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None
        };

        // Load the TesTask JSON file
        let task_json = std::fs::read_to_string("/home/aarav/dev/ga4gh-sdk/lib/sample/grape.tes").expect("Unable to read file");
        let task: TesTask = serde_json::from_str(&task_json).expect("JSON was not well-formatted");

        // Create the service and Tes instance
        let service = Service { 
            base_url: "http://localhost:8000".to_string(),
            client: client_tes.clone(),
            username: None,
            password: None,
            token: None,

         };
        let tes = Tes::new(service);

        // Create the parameters
        let params = CreateTaskParams { body: task };

        // Call the create_task method
        let result = tes.create_task(&config, params).await;
        println!("{:?}", result);
    }
}