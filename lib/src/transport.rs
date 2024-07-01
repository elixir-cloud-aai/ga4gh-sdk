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
        let full_url = format!("{}{}", self.config.base_path, endpoint);
        let url = reqwest::Url::parse(&full_url);
        if url.is_err() {
            error!(
                "Invalid endpoint (shouldn't contain base url): {}",
                endpoint
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid endpoint",
            )));
        }

        let mut request_builder = self.client.request(method, &full_url).header(
            reqwest::header::USER_AGENT,
            self.config.user_agent.clone().unwrap_or_default(),
        );

        if let Some(ref params_value) = params {
            request_builder = request_builder.query(params_value);
        }

        if let Some(ref data_value) = data {
            // Figure out some way to filter out `Null` values of data_value
            request_builder = request_builder.json(&data_value);
        }

        let resp = request_builder.send().await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(content)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                content,
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

        let config = Configuration::new(base_url.clone(), None, None);
        let transport = Transport::new(&config.clone());
        let response = transport.get("/test", None).await;

        assert!(response.is_ok());
        let body = response.unwrap();
        assert_eq!(body, r#"{"message": "success"}"#);
    }
}
