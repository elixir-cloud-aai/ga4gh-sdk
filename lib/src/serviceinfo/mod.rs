use crate::transport::Transport;
use crate::configuration::ServiceConfiguration;

pub struct ServiceInfo {
    transport: Transport,
}

impl ServiceInfo {
    pub fn new(config: &ServiceConfiguration) -> Self {
        // todo: read service info from the server
        ServiceInfo {
            transport: Transport::new(&config.clone()),
        }
    }
}