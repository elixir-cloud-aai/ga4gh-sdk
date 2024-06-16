use reqwest::{Client, Response};
use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::fmt;


use crate::{task_execution_service::ResponseContent, task_execution_service::models};

pub enum CreateTaskError {
    UnknownValue(serde_json::Value),
}

#[derive(Debug)]
struct MyError {
    message: String,
}

impl MyError {
    fn new(message: String) -> MyError {
        MyError {
            message,
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyError {}


#[derive(Debug, Clone)]
pub struct Transport {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    pub password: Option<String>, //CHECK IF REQUIRED
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}



impl Transport {
    pub fn new(base_path: String, user_agent: Option<String>, password: Option<String>, bearer_access_token: Option<String>) -> Self {
        Transport {
            base_path,
            user_agent,
            client: Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token,
            api_key: None,
            password,
        }
    }

    pub async fn request(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        data: Option<Value>,
        _params: Option<Value>, // CHECK IF IT CAN BE USED ANYWHERE/ How to use it
    ) -> Result<String, Box<dyn Error>> {
        let mut req_builder = self.client.request(method, endpoint);

        if let Some(ref user_agent) = self.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        
        req_builder = req_builder.json(&data);
        // req_builder = req_builder.json(&params);

        let req = req_builder.build()?;
        let resp = self.client.execute(req).await?;

        let status = resp.status();
        let content = resp.text().await?;

        if status.is_success() {
            Ok(content)
        } else {
            Err(Box::new(MyError::new(content)))
        }
    }

}


impl Default for Transport {
    fn default() -> Self {
        Transport {
            base_path: "/ga4gh/tes/v1".to_owned(),
            user_agent: Some("OpenAPI-Generator/1.1.0/rust".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
            password: None,
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::transport::Transport;
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

        let transport = Transport::new(base_url.clone(), None, None, None);

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

        let transport = Transport::new(base_url.clone(), None, None, None);

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