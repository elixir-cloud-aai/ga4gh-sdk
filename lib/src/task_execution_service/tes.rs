use reqwest;

use crate::{task_execution_service::{models, ResponseContent}, transport};
use super::{Error, configuration};
use crate::transport::Transport;
use serde_json::json;
use crate::task_execution_service::models::TesCreateTaskResponse;


// Defining service class
pub struct Tes<'a> {
    transport: &'a Transport, // http client essentially 
    // service_info: // service model
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

impl<'a> Tes<'a> {
    pub fn new(transport: &'a Transport) -> Self {
        // todo retrieve serviceinfo (incl the version), double check that it's TES
        // service => transport
        Tes { transport }
    }

    /// Create a new task. The user provides a Task document, which the server uses as a basis and adds additional fields.
    // pub async fn create(&self, configuration: &configuration::Configuration, params: CreateTaskParams) -> Result<models::TesCreateTaskResponse, Error<CreateTaskError>> {
    pub async fn create(&self, transport: &transport::Transport, params: CreateTaskParams) {
        let local_var_configuration = transport;

        // unbox the parameters
        let body = params.body;
        
        let local_var_uri_str = format!("{}/tasks", local_var_configuration.base_path);
        let endpoint = local_var_uri_str.as_str();
        let response = self.transport.request(reqwest::Method::POST, endpoint, Some(serde_json::to_value(body).unwrap()), None).await;
        
        match response {
            Ok(content) => {
                // Handle the successful response
                // let content = &response.text().await?;
                // let task_response: TesCreateTaskResponse = serde_json::from_str(&content)?;
                println!("Success; {}", content); // TODO log functions
                // Ok(task_response)
            },
            Err(e) => {
                // Err(CreateTaskError::from(e))
                // Handle the error
                eprintln!("Error: {}", e);
                // You can also process the error as needed
            },
        }
        // let status = response.status();

        // let content = response.text().await?;

        // if !status.is_success() {
        //     let error_value: serde_json::Value = serde_json::from_str(&content)?;
        //     return Err(Error::ResponseError(ResponseContent {
        //         status,
        //         content: CreateTaskError::UnknownValue(error_value),
        //     }));
        // }

        // let response_data: TesCreateTaskResponse = serde_json::from_str(&content)?;
        // Ok(response_data)

    }

    // TODO: pub fn status()

    // TODO: pub fn list()
}

#[cfg(test)]
mod tests {
    use reqwest::header::TRAILER;

    use super::*;
    use crate::task_execution_service::configuration::Configuration;
    use crate::task_execution_service::tes::reqwest::Client;
    use crate::task_execution_service::models::TesTask;

    #[tokio::test]
    async fn test_create_task() {
        let client_tes = Client::new();

        // Define the configuration
        let transport = Transport {
            base_path: "http://localhost:8000".to_string(),
            user_agent: None,
            client: client_tes.clone(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
            password: None
        };

        // Load the TesTask JSON file
        let task_json = std::fs::read_to_string("/home/aarav/dev/ga4gh-sdk/lib/sample/grape.tes").expect("Unable to read file");
        let task: TesTask = serde_json::from_str(&task_json).expect("JSON was not well-formatted");

        
        let tes = Tes::new(&transport);

        // Create the parameters
        let params = CreateTaskParams { body: task };

        // Call the create_task method
        let result = tes.create(&transport, params).await;
        println!("{:?}", result);
    }
}