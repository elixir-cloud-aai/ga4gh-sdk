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
    pub async fn create_task(&self, configuration: &configuration::Configuration, params: CreateTaskParams) -> Result<models::TesCreateTaskResponse, Error<CreateTaskError>> {
        let local_var_configuration = configuration;

        // unbox the parameters
        let body = params.body;
        
        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/tasks", local_var_configuration.base_path);
        let endpoint = local_var_uri_str.as_str();
        let mut response_ = self.service.request(reqwest::Method::POST, endpoint, Some(serde_json::to_value(body).unwrap()), None);


        // if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        //     serde_json::from_str(&local_var_content).map_err(Error::from)
        // } else {
        //     let local_var_entity: Option<CreateTaskError> = serde_json::from_str(&local_var_content).ok();
        //     let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        //     Err(Error::ResponseError(local_var_error))
        // }


        // This is not correct, I am working on this
        match self.service.request(reqwest::Method::POST, endpoint, Some(serde_json::to_value(body).unwrap()), None).await {
            Ok(response_text) => {
                match serde_json::from_str::<TesCreateTaskResponse>(&response_text) {
                    Ok(parsed_response) => Ok(parsed_response),
                    Err(_) => Err(Error::new(CreateTaskError::UnknownValue(json!({
                        "error": "Failed to parse response",
                        "response": response_text
                    })))),
                }
            }
            Err(e) => Err(Error::new(CreateTaskError::UnknownValue(json!({
                "error": e.to_string(),
                "cause": "Request execution failed"
            })))),
        }
    }

 
}