pub mod models;

use crate::clients::serviceinfo::models::Service;
use crate::clients::serviceinfo::ServiceInfo;
use crate::clients::wes::models::WesRunId;
use crate::clients::wes::models::WesRunListResponse;
use crate::clients::wes::models::WesRunLog;
use crate::clients::wes::models::WesRunRequest;
use crate::clients::wes::models::WesState;
use crate::utils::configuration::Configuration;
use crate::utils::transport::Transport;
use log::error;
use serde_json::from_str;
use serde_json::json;

/// URL-encodes a string.
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

#[derive(Debug, Clone)]
pub struct Run {
    /// The unique ID of the run.
    pub id: String,
    /// The transport layer for sending HTTP requests.
    pub transport: Transport,
}

impl Run {
    /// Creates a new `Run` instance.
    pub fn new(id: String, transport: Transport) -> Self {
        Run { id, transport }
    }

    /// Fetches the current status of the run.
    pub async fn status(&self) -> Result<WesState, Box<dyn std::error::Error>> {
        let run_id = &self.id;
        let url = format!("/runs/{}/status", run_id);
        let response = self.transport.get(&url, None).await;
        match response {
            Ok(resp_str) => {
                let status: serde_json::Value = from_str(&resp_str)?;
                // The spec says GET /runs/{run_id}/status returns RunStatus which has run_id and state.
                let state_str = status
                    .get("state")
                    .and_then(|s| s.as_str())
                    .ok_or("Missing state field")?;
                let state: WesState = serde_json::from_value(json!(state_str))?;
                Ok(state)
            }
            Err(e) => {
                let err_msg = format!("HTTP request failed: {}", e);
                error!("{}", err_msg);
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    err_msg,
                )))
            }
        }
    }

    /// Cancels the run.
    pub async fn cancel(&self) -> Result<WesRunId, Box<dyn std::error::Error>> {
        let id = &self.id;
        let id = urlencode(id);
        let url = format!("/runs/{}/cancel", id);
        let response = self.transport.post(&url, None).await;
        match response {
            Ok(resp_str) => {
                let run_id: WesRunId = from_str(&resp_str)?;
                Ok(run_id)
            }
            Err(e) => Err(format!("HTTP request failed: {}", e).into()),
        }
    }

    /// Fetches the details of the run (including logs and outputs).
    pub async fn log(&self) -> Result<WesRunLog, Box<dyn std::error::Error>> {
        let id = &self.id;
        let url = format!("/runs/{}", id);
        let response = self.transport.get(&url, None).await;
        match response {
            Ok(resp_str) => {
                let log: WesRunLog = from_str(&resp_str)?;
                Ok(log)
            }
            Err(e) => Err(e),
        }
    }
}

/// The main struct for interacting with a WES service.
#[derive(Debug)]
pub struct WES {
    #[allow(dead_code)]
    pub config: Configuration,
    pub service: Result<Service, Box<dyn std::error::Error>>,
    pub transport: Transport,
}

impl WES {
    /// Creates a new `WES` instance.
    pub async fn new(config: &Configuration) -> Result<Self, Box<dyn std::error::Error>> {
        let transport = Transport::new(config);
        let service_info = ServiceInfo::new(config)?;

        let resp = service_info.get().await;

        let instance = WES {
            config: config.clone(),
            transport,
            service: resp,
        };

        instance.check()?;
        Ok(instance)
    }

    fn check(&self) -> Result<(), String> {
        let resp = &self.service;
        match resp.as_ref() {
            Ok(service) if service.r#type.artifact == "wes" => Ok(()),
            Ok(_) => Err("The endpoint is not an instance of WES".into()),
            Err(_) => Err("Error accessing the service".into()),
        }
    }

    /// Creates a new workflow run.
    pub async fn run_workflow(
        &self,
        request: WesRunRequest,
    ) -> Result<Run, Box<dyn std::error::Error>> {
        self.check().map_err(|e| {
            error!("Service check failed: {}", e);
            e
        })?;

        let response = self
            .transport
            .post("/runs", Some(json!(request)))
            .await;

        match response {
            Ok(response_body) => {
                let v: serde_json::Value = serde_json::from_str(&response_body)?;
                let run_id = v
                    .get("run_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                let run = Run {
                    id: run_id,
                    transport: self.transport.clone(),
                };
                Ok(run)
            }
            Err(e) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create run: {}", e),
            ))),
        }
    }

    /// List runs.
    pub async fn list_runs(
        &self,
        next_page_token: Option<String>,
        page_size: Option<i64>,
    ) -> Result<WesRunListResponse, Box<dyn std::error::Error>> {
        // Build query params
        let mut query = Vec::new();
        if let Some(token) = next_page_token {
            query.push(format!("page_token={}", token));
        }
        if let Some(size) = page_size {
            query.push(format!("page_size={}", size));
        }
        let url = if query.is_empty() {
            "/runs".to_string()
        } else {
            format!("/runs?{}", query.join("&"))
        };

        let response = self.transport.get(&url, None).await;

        match response {
            Ok(resp_str) => {
                let list: WesRunListResponse = from_str(&resp_str)?;
                Ok(list)
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

    /// Get run full details (wrapper around Run::log for convenience from client)
    pub async fn get_run(&self, id: &str) -> Result<WesRunLog, Box<dyn std::error::Error>> {
        let run = Run::new(id.to_string(), self.transport.clone());
        run.log().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::serviceinfo::models::ServiceType;
    use mockito::mock;
    use mockito::server_url;
    use url::Url;

    #[tokio::test]
    async fn test_wes_create() {
        let _m = mock("POST", "/runs")
            .with_status(200)
            .with_body(r#"{"run_id": "123"}"#)
            .create();

        let mock_url = Url::parse(&server_url()).expect("Invalid URL");
        let config = Configuration::new(mock_url);
        let transport = Transport::new(&config);

        let wes = WES {
            config,
            service: Ok(Service {
                r#type: Box::new(ServiceType {
                    artifact: "wes".to_string(),
                    ..Default::default()
                }),
                ..Service::default()
            }),
            transport,
        };

        let request = WesRunRequest::default();
        let result = wes.run_workflow(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, String::from("123"));
    }

    #[tokio::test]
    async fn test_run_status() {
        let _m = mock("GET", "/runs/123/status")
            .with_status(200)
            .with_body(r#"{"run_id": "123", "state": "COMPLETE"}"#)
            .create();

        let mock_url = Url::parse(&server_url()).expect("Invalid URL");
        let transport = Transport::new(&Configuration::new(mock_url));
        let run = Run::new("123".to_string(), transport);

        let result = run.status().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), WesState::Complete);
    }

    #[tokio::test]
    async fn test_run_cancel() {
        let _m = mock("POST", "/runs/123/cancel")
            .with_status(200)
            .with_body(r#"{"run_id": "123"}"#)
            .create();

        let mock_url = Url::parse(&server_url()).expect("Invalid URL");
        let transport = Transport::new(&Configuration::new(mock_url));
        let run = Run::new("123".to_string(), transport);

        let result = run.cancel().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().run_id, Some("123".to_string()));
    }

    #[tokio::test]
    async fn test_wes_list_runs() {
        let _m = mock("GET", "/runs")
            .with_status(200)
            .with_body(r#"{"runs": [], "next_page_token": ""}"#)
            .create();

        let mock_url = Url::parse(&server_url()).expect("Invalid URL");
        let config = Configuration::new(mock_url);
        let transport = Transport::new(&config);
        let wes = WES {
            config,
            service: Ok(Service::default()),
            transport,
        };

        let result = wes.list_runs(None, None).await;
        assert!(result.is_ok());
        let runs = result.unwrap().runs;
        assert!(runs.is_some());
        assert!(runs.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_wes_get_run() {
        let _m = mock("GET", "/runs/123")
            .with_status(200)
            .with_body(r#"{"run_id": "123", "state": "COMPLETE"}"#)
            .create();

        let mock_url = Url::parse(&server_url()).expect("Invalid URL");
        let config = Configuration::new(mock_url);
        let transport = Transport::new(&config);
        let wes = WES {
            config,
            service: Ok(Service::default()),
            transport,
        };

        let result = wes.get_run("123").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().run_id, Some("123".to_string()));
    }
}
