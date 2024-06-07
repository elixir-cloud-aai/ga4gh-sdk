use reqwest;

use crate::{tes::ResponseContent, tes::models};
use super::{Error, configuration};

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

/// Create a new task. The user provides a Task document, which the server uses as a basis and adds additional fields.
pub async fn create_task(configuration: &configuration::Configuration, params: CreateTaskParams) -> Result<models::TesCreateTaskResponse, Error<CreateTaskError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let body = params.body;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tasks", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateTaskError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}