use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::fmt;


use crate::{tes::ResponseContent, tes::models};

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

#[derive(Debug)]
pub struct Service {
    base_url: String,
    client: Client,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
}

impl Service {
    pub fn new(base_url: String, username: Option<String>, password: Option<String>, token: Option<String>) -> Self {
        Service {
            base_url,
            client: Client::new(),
            username,
            password,
            token,
        }
    }

    pub async fn request(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        data: Option<Value>,
        params: Option<Value>,
    ) -> Result<String, Box<dyn Error>> {
        let mut local_var_req_builder = self.client.request(reqwest::Method::POST, endpoint);

        if let Some(ref local_var_user_agent) = self.username {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        local_var_req_builder = local_var_req_builder.json(&data);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = self.client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if local_var_status.is_success() {
            Ok(local_var_content)
        } else {
            Err(Box::new(MyError::new(local_var_content)))

        }
    }

}