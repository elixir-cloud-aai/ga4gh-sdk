use reqwest::{Client, Response};
use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::fmt;
use crate::configuration::Configuration;

// note: could implement custom certs handling, such as in-TEE generated ephemerial certs

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
        let resp = self.client
            .request(method, endpoint)
            .header(reqwest::header::USER_AGENT, self.config.user_agent.clone().unwrap_or_default())
            .json(&data)
            .query(&params)
            .send()
            .await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(content)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, content)))
        }
    }

    pub async fn get(&self, endpoint: &str, params: Option<Value>) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::GET, endpoint, None, params).await
    }

    pub async fn post(&self, endpoint: &str, data: Value) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::POST, endpoint, Some(data), None).await
    }

    pub async fn put(&self, endpoint: &str, data: Value) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::PUT, endpoint, Some(data), None).await
    }

    pub async fn delete(&self, endpoint: &str) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::DELETE, endpoint, None, None).await
    }

    // other HTTP methods can be added here
}


// impl Default for Transport {
//     fn default() -> Self {
//         Transport {
//             base_path: "/ga4gh/tes/v1".to_owned(),
//             user_agent: Some("OpenAPI-Generator/1.1.0/rust".to_owned()),
//             client: reqwest::Client::new(),
//             basic_auth: None,
//             oauth_access_token: None,
//             bearer_access_token: None,
//             api_key: None,
//             password: None,
//         }
//     }
// }



#[cfg(test)]
mod tests {
    use crate::{configuration::Configuration, transport::Transport};
    use reqwest::Method;
    use serde_json::json;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn test_request_success() {
        let base_url = &mockito::server_url();
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "success"}"#)
            .create();

        let config= Configuration::new(base_url.clone(), None, None);

        let transport = Transport::new(&config.clone());

        let response = transport.request(
            Method::GET,
            &format!("{}/test", base_url),
            None,
            None,
        ).await;

        assert!(response.is_ok());
        let body = response.unwrap();
        assert_eq!(body, r#"{"message": "success"}"#);
    }

    #[tokio::test]
    async fn test_request_failure() {
        let base_url = &mockito::server_url();
        let _m = mock("GET", "/test")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "not found"}"#)
            .create();

        let config= Configuration::new(base_url.clone(), None, None);

        let transport = Transport::new(&config.clone());

        let response = transport.request(
            Method::GET,
            &format!("{}/test", base_url),
            None,
            None,
        ).await;

        assert!(response.is_err());
        let error = response.err().unwrap();
        assert_eq!(error.to_string(), r#"{"message": "not found"}"#);
    }
}