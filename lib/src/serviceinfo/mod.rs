pub mod models;
use crate::configuration::Configuration;
use crate::transport::Transport;

#[derive(Clone)]
pub struct ServiceInfo {
    transport: Transport,
}

impl ServiceInfo {
    pub fn new(config: &Configuration) -> Result<Self, Box<dyn std::error::Error>> {
        let transport = &Transport::new(config);
        let instance = ServiceInfo {
            transport: transport.clone(),
        };
        Ok(instance)
    }

    pub async fn get(&self) -> Result<models::Service, Box<dyn std::error::Error>> {
        let response = self.transport.get("/service-info", None).await;
        match response {
            Ok(response_body) => match serde_json::from_str::<models::Service>(&response_body) {
                Ok(service) => Ok(service),
                Err(e) => {
                    log::error!("Failed to deserialize response: {}", e);
                    Err(e.into())
                }
            },
            Err(e) => {
                log::error!("Error: {}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::Configuration;
    use crate::serviceinfo::ServiceInfo;
    use crate::test_utils::{ensure_funnel_running, setup};
    use tokio;

    #[tokio::test]
    async fn test_get_service_info_from_funnel() {
        setup();
        let mut config = Configuration::default();
        let funnel_url = ensure_funnel_running().await;
        config.set_base_path(&funnel_url);
        let service_info = ServiceInfo::new(&config).unwrap();

        // Call get_service_info and print the result
        match service_info.get().await {
            Ok(service) => {
                println!("Service Info: {:?}", service);
            }
            Err(e) => {
                println!("Failed to get service info: {}", e);
            }
        }
    }
}
