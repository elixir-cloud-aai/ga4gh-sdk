use crate::configuration::Configuration;
use log::error;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;

// note: could implement custom certs handling, such as in-TEE generated ephemerial certs
#[derive(Clone, Debug)]
pub struct Transport {
    pub config: Configuration,
    pub client: reqwest::Client,
}

impl Transport {
    pub fn new(config: &Configuration) -> Self {
        Transport {
            config: config.clone(),
            client: Client::new(),
        }
    }

    async fn request(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        data: Option<Value>,
        params: Option<Value>,
    ) -> Result<String, Box<dyn Error>> {
        let base_url = reqwest::Url::parse(&self.config.base_path)?;
        let url = base_url.join(endpoint).map_err(|e| {
            error!("Invalid endpoint (shouldn't contain base url): {}. Error: {}", endpoint, e);
        let url = reqwest::Url::parse(&full_url).map_err(|e| {
            error!("Invalid endpoint (shouldn't contain base url): {}. Error: {}", endpoint, e);
            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid endpoint")) as Box<dyn std::error::Error>
        })?;

        let mut request_builder = self.client.request(method, url);

        if let Some(ref user_agent) = self.config.user_agent {
            request_builder = request_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        if let Some(ref params_value) = params {
            // Validate or log params_value before setting it as query parameters
            if params_value.is_object() {
                request_builder = request_builder.query(params_value);
            } else {
                error!("params_value is not an object and cannot be used as query parameters: {:?}", params_value);
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "params_value must be an object")));
            }
        }

        if let Some(ref data) = data {
            if serde_json::to_string(&data).is_ok() {
                request_builder = request_builder.json(&data);
            } else {
                log::error!("Parameters are invalid, and can't convert to JSON");
            }
        }

        let resp = request_builder.send().await.map_err(|e| {
	            eprintln!("HTTP request failed: {}", e);
	            e
	        })?;

        let status = resp.status();
        let content = resp.text().await.map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Failed to read response text: {}", e))
        })?;

        if status.is_success() {
            Ok(content)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Request failed with status: {}. Response: {}", status, content),
            )))
        }
    }

    pub async fn get(
        &self,
        endpoint: &str,
        params: Option<Value>,
    ) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::GET, endpoint, None, params)
            .await
    }

    pub async fn post(
        &self,
        endpoint: &str,
        data: Option<Value>,
    ) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::POST, endpoint, data, None)
            .await
    }

    pub async fn put(&self, endpoint: &str, data: Value) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::PUT, endpoint, Some(data), None)
            .await
    }

    pub async fn delete(&self, endpoint: &str) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::DELETE, endpoint, None, None)
            .await
    }

    // other HTTP methods can be added here
}

#[cfg(test)]
mod tests {
    use crate::configuration::Configuration;
    use crate::test_utils::setup;
    use crate::transport::Transport;
    use mockito::mock;

    #[tokio::test]
    async fn test_request() {
        setup();
        let base_url = &mockito::server_url();
        // effectively no sense in testing various responses, as it's reqwest's responsibility
        // we should test Transport's methods
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "success"}"#)
            .create();

        let config = Configuration::new(base_url.clone(),None, None, None);
        let transport = Transport::new(&config.clone());
        let response = transport.get("/test", None).await;

        assert!(response.is_ok());
        let body = response.unwrap();
        assert_eq!(body, r#"{"message": "success"}"#);
    }
}
