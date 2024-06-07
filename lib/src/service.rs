use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;

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
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);

        if let Some(token) = &self.token {
            headers.insert(
                "Authorization",
                format!("Bearer {}", token).parse()?,
            );
        }

        let mut req_builder = self.client.request(method, &url).headers(headers);

        if let Some(data) = data {
            req_builder = req_builder.json(&data);
        }

        if let Some(params) = params {
            req_builder = req_builder.query(&params);
        }

        let response = req_builder.send().await?;

        if !response.status().is_success() {
            let error_message = format!("Error: HTTP {} - {}", response.status(), response.status().canonical_reason().unwrap_or("Unknown error"));
            eprintln!("{}", error_message);
            return Err(error_message.into());
        }

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        let response_data = if content_type.contains("application/json") {
            response.json::<Value>().await?
        } else {
            Value::String(response.text().await?)
        };

        Ok(response_data)
    }

}
