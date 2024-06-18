

// pub struct ServiceInfo {
//     transport: Transport;
// }

// impl ServiceInfo {

// }

use reqwest;

use crate::{serviceinfo::{models, ResponseContent}, transport};
use super::Error;
use crate::configuration;


/// struct for typed errors of method [`get_service_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceInfoError {
    UnknownValue(serde_json::Value),
}


pub async fn get_service_info(transport: &transport::Transport) -> Result<models::Service, Box<dyn std::error::Error>> {
    
    let client = &transport.client;
    let configuration = &transport.config;

    let url = format!("{}/service-info", configuration.base_path);
    let response = &transport.get(&url,None).await;
        match response {
            Ok(response_body) => {
                match serde_json::from_str::<models::Service>(&response_body) {
                    Ok(tes_create_task_response) => Ok(tes_create_task_response),
                    Err(e) => {
                        log::error!("Failed to deserialize response: {}", e);
                        Err("Failed to deserialize response".into())
                    },
                }
            },
            Err(e) => {
                log::error!("Error: {}", e);
                Err(e.into())
            },
        }
    // let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    // if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
    //     local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    // }
    // if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
    //     local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    // };

    // let local_var_req = local_var_req_builder.build()?;
    // let local_var_resp = local_var_client.execute(local_var_req).await?;

    // let local_var_status = local_var_resp.status();
    // let local_var_content = local_var_resp.text().await?;

    // if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
    //     serde_json::from_str(&local_var_content).map_err(Error::from)
    // } else {
    //     let local_var_entity: Option<GetServiceInfoError> = serde_json::from_str(&local_var_content).ok();
    //     let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
    //     Err(Error::ResponseError(local_var_error))
    // }
}
