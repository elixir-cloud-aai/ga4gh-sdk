use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;


use crate::{tes::ResponseContent, tes::models};

pub enum CreateTaskError {
    UnknownValue(serde_json::Value),
}

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
    ) -> Result<Value, Box<dyn Error>> {
        // What client are we using?
        let mut local_var_req_builder = self.client.request(reqwest::Method::POST, endpoint);

        if let Some(ref local_var_user_agent) = self.username {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        // Check what are data and params
        local_var_req_builder = local_var_req_builder.json(&data);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = self.client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;


        // Check what to return, whether Result<Value, Box<dyn Error>> or not
        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(|e| Box::new(e) as Box<dyn Error>)
            // serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            // let local_var_entity: Option<CreateTaskError> = serde_json::from_str(&local_var_content).ok();
            // let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            // Err(Error::ResponseError(local_var_error))
            let error_message = format!("Error: HTTP {} - {}", local_var_status, local_var_status.canonical_reason().unwrap_or("Unknown error"));
            eprintln!("{}", error_message);
            Err(error_message.into())
        }

        //GA4GH-CLI PYTHON CODE CONVERTED TO RUST (OLD CODE)

        
        // let url = format!("{}/{}", self.base_url, endpoint);
        // let mut headers = reqwest::header::HeaderMap::new();
        // headers.insert("Content-Type", "application/json".parse()?);

        // if let Some(token) = &self.token {
        //     headers.insert(
        //         "Authorization",
        //         format!("Bearer {}", token).parse()?,
        //     );
        // }

        // let mut req_builder = self.client.request(method, &url).headers(headers);

        // if let Some(data) = data {
        //     req_builder = req_builder.json(&data);
        // }

        // if let Some(params) = params {
        //     req_builder = req_builder.query(&params);
        // }

        // let response = req_builder.send().await?;

        // if !response.status().is_success() {
        //     let error_message = format!("Error: HTTP {} - {}", response.status(), response.status().canonical_reason().unwrap_or("Unknown error"));
        //     eprintln!("{}", error_message);
        //     return Err(error_message.into());
        // }

        // let content_type = response
        //     .headers()
        //     .get(reqwest::header::CONTENT_TYPE)
        //     .and_then(|value| value.to_str().ok())
        //     .unwrap_or("");

        // let response_data = if content_type.contains("application/json") {
        //     response.json::<Value>().await?
        // } else {
        //     Value::String(response.text().await?)
        // };

        // Ok(response_data)
    }

}

