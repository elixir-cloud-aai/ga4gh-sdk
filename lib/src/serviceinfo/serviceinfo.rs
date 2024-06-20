
use reqwest;

use crate::{serviceinfo::{models, ResponseContent, Transport}, transport};
use super::Error;
use crate::configuration;


/// struct for typed errors of method [`get_service_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetServiceInfoError {
    UnknownValue(serde_json::Value),
}


pub struct ServiceInfo {
    transport: Transport,
}

impl ServiceInfo {
    pub fn new(transport: Transport)-> Self{
        ServiceInfo { transport }
    }
    pub async fn get_service_info(&self) -> Result<models::Service, Box<dyn std::error::Error>> {
        
        let configuration = &self.transport.config;

        let url = format!("{}/service-info", configuration.base_path);
        let response = self.transport.get(&url,None).await;
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
                    Err(e)
                },
            }
    }

}
