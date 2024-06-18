use crate::transport::Transport;
use crate::configuration::Configuration;

pub struct ServiceInfo {
    transport: Transport,
}

impl ServiceInfo {
    pub fn new(config: &Configuration) -> Self {
        // todo: read service info from the server
        ServiceInfo {
            transport: Transport::new(&config.clone()),
        }
    }
}