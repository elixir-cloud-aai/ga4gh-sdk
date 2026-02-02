pub mod models;

use crate::utils::configuration::Configuration;
use crate::clients::serviceinfo::models::Service;
use crate::clients::serviceinfo::ServiceInfo;
use crate::clients::drs::models::DrsObject;
use crate::clients::drs::models::AccessUrl;
use crate::utils::transport::Transport;
use serde_json::from_str;
use log::error;

/// The main struct for interacting with a DRS service.
#[derive(Debug)]
pub struct DRS {
    #[allow(dead_code)]
    pub config: Configuration,
    pub service: Result<Service, Box<dyn std::error::Error>>,
    pub transport: Transport,
}

impl DRS {
    /// Creates a new `DRS` instance.
    ///
    /// # Arguments
    /// - `config`: A reference to the service configuration.
    ///
    /// # Returns
    /// - A new `DRS` instance, or an error if the initialization fails.
    pub async fn new(config: &Configuration) -> Result<Self, Box<dyn std::error::Error>> {
        let transport = Transport::new(config);
        let service_info = ServiceInfo::new(config)?;
        let resp = service_info.get().await;

        let instance = DRS {
            config: config.clone(),
            transport,
            service: resp,
        };

        instance.check()?;
        Ok(instance)
    }

    /// Checks if the service is of DRS class.
    ///
    /// # Returns
    /// - Ok(()) if the service is valid.
    /// - Err(String) if the service is invalid or an error occurs.
    fn check(&self) -> Result<(), String> {
        let resp = &self.service;
        match resp.as_ref() {
            Ok(service) if service.r#type.artifact == "drs" => Ok(()),
            Ok(_) => Err("The endpoint is not an instance of DRS".into()),
            Err(_) => Err("Error accessing the service".into()),
        }
    }

    /// Retrieves the details of a specific DRS object.
    ///
    /// # Arguments
    /// - `object_id`: The ID of the object to retrieve.
    ///
    /// # Returns
    /// - On success, returns a `DrsObject` containing the object details.
    /// - On failure, returns an error.
    pub async fn get_object(&self, object_id: &str) -> Result<DrsObject, Box<dyn std::error::Error>> {
        let url = format!("/ga4gh/drs/v1/objects/{}", object_id);
        let response = self.transport.get(&url, None).await;

        match response {
            Ok(resp_str) => {
                let object: DrsObject = from_str(&resp_str)?;
                Ok(object)
            }
            Err(e) => {
                error!("HTTP request failed: {:?}", e);
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("HTTP request failed: {:?}", e),
                )))
            }
        }
    }

    /// Retrieves an access URL for a specific DRS object.
    ///
    /// # Arguments
    /// - `object_id`: The ID of the object.
    /// - `access_id`: The access ID.
    ///
    /// # Returns
    /// - On success, returns an `AccessUrl` containing the URL and headers.
    /// - On failure, returns an error.
    pub async fn get_access_url(&self, object_id: &str, access_id: &str) -> Result<AccessUrl, Box<dyn std::error::Error>> {
        let url = format!("/ga4gh/drs/v1/objects/{}/access/{}", object_id, access_id);
        let response = self.transport.get(&url, None).await;

        match response {
            Ok(resp_str) => {
                let access_url: AccessUrl = from_str(&resp_str)?;
                Ok(access_url)
            }
            Err(e) => {
                error!("HTTP request failed: {:?}", e);
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("HTTP request failed: {:?}", e),
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::serviceinfo::models::ServiceType;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_get_object() {
        let object_id = "123";
        let response_body = r#"{
            "id": "123",
            "self_uri": "drs://drs.example.org/123",
            "size": 1024,
            "created_time": "2023-01-01T00:00:00Z",
            "checksums": []
        }"#;

        let _m = mock("GET", "/ga4gh/drs/v1/objects/123")
            .with_status(200)
            .with_body(response_body)
            .create();

        let mock_url = url::Url::parse(&server_url()).expect("Invalid URL");
        let config = Configuration::new(mock_url);
        let transport = Transport::new(&config);

        let drs = DRS {
            config,
            service: Ok(Service {
                r#type: Box::new(ServiceType {
                    artifact: "drs".to_string(),
                    ..Default::default()
                }),
                ..Service::default()
            }),
            transport,
        };

        let result = drs.get_object(object_id).await;
        assert!(result.is_ok());
        let object = result.unwrap();
        assert_eq!(object.id, "123");
        assert_eq!(object.size, 1024);
    }

    #[tokio::test]
    async fn test_get_access_url() {
        let object_id = "123";
        let access_id = "access123";
        let response_body = r#"{
            "url": "http://example.com/data",
            "headers": ["Authorization: Basic Z2E0Z2g6ZHJz"]
        }"#;

        let _m = mock("GET", "/ga4gh/drs/v1/objects/123/access/access123")
            .with_status(200)
            .with_body(response_body)
            .create();

        let mock_url = url::Url::parse(&server_url()).expect("Invalid URL");
        let config = Configuration::new(mock_url);
        let transport = Transport::new(&config);

        let drs = DRS {
            config,
            service: Ok(Service {
                r#type: Box::new(ServiceType {
                    artifact: "drs".to_string(),
                    ..Default::default()
                }),
                ..Service::default()
            }),
            transport,
        };

        let result = drs.get_access_url(object_id, access_id).await;
        assert!(result.is_ok());
        let access_url = result.unwrap();
        assert_eq!(access_url.url, "http://example.com/data");
        assert_eq!(access_url.headers.unwrap()[0], "Authorization: Basic Z2E0Z2g6ZHJz");
    }
}
