/// A struct representing a transport for making HTTP requests.
///
/// The `Transport` struct is responsible for handling HTTP requests using the `reqwest` crate.
/// It provides methods for making GET, POST, PUT, and DELETE requests.
/// It also supports extensible TLS verifier methods for enhanced security.
///
/// The `Transport` struct takes a `Configuration` as a parameter, which provides:
/// - The base path for the API.
/// - Credentials such as basic authentication, OAuth access tokens, and API keys.
/// - User agent information.
/// - Additional configuration details for service extensions.
///
/// # Examples
///
/// ```rust
/// use crate::ga4gh_sdk::utils::configuration::Configuration;
/// use crate::ga4gh_sdk::utils::transport::Transport;
/// use url::Url;
/// use serde_json::json;
///
/// let config = Configuration::new(Url::parse("https://api.example.com").unwrap());
/// let transport = Transport::new(&config);
///
/// // Make a GET request
/// async {
///     let response = transport.get("/users", None).await;
/// };
///
/// // Make a POST request
/// async {
///     let data = json!({"name": "John Doe", "age": 30});
///     let response = transport.post("/users", Some(data)).await;
/// };
///
/// // Make a PUT request
/// async {
///     let data = json!({"name": "John Doe", "age": 30});
///     let response = transport.put("/users/1", data).await;
/// };
///
/// // Make a DELETE request
/// async {
///     let response = transport.delete("/users/1").await;
/// };
/// ```
use crate::utils::configuration::Configuration;
use log::{debug, error, info};
use reqwest::ClientBuilder;
use serde_json::{Value, json};
use std::error::Error;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

#[derive(Clone, Debug)]
pub struct Transport {
    pub config: Configuration,
    pub client: reqwest::Client,
}

#[derive(Debug, Deserialize, Serialize)]
struct TLSVerifierResponse {
    message: Option<String>,
    certificate: String,
    #[serde(rename = "verification-result")]
    verification_result: bool,
}

impl Transport {

    /// Creates a new `Transport` instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the transport.
    ///
    /// # Returns
    ///
    /// A new `Transport` instance.
    pub fn new(config: &Configuration) -> Result<Self, Box<dyn Error>> {
        let mut certificates = Vec::new();
        let tls_methods = config.extensions_manager.lookup_extension_methods("tls-verifier");
        for tls_method in tls_methods {
            let service_host = config.base_path.host_str().unwrap();
            let extension_config = config.extensions.as_ref().unwrap().get_extension_config(&tls_method.extension_name);

            let tls_method_param = json!({
                "service-host": service_host,
                "extension-config": extension_config,
            });
            debug!("Calling TLS extension method: {}", tls_method.internal_name);

            let response_json = unsafe { (tls_method.method)(tls_method_param) };
            // Parse response and handle double-encoded JSON
            let response: TLSVerifierResponse = serde_json::from_str(&response_json.as_str().unwrap_or_default())
                .and_then(|json: Value| {
                    if let Some(inner) = json.as_str() {
                        serde_json::from_str(inner)
                    } else {
                        serde_json::from_value(json)
                    }
                })
                .map_err(|e| {
                    debug!("Error parsing TLS extension method response: {}", e);
                    Box::new(e) as Box<dyn std::error::Error>
                })?;
            debug!("TLS extension method response: {:#?}", response);

            const SECURITY_MODE_ENFORCE : &str = "enforce";
            const SECURITY_MODE_PERMISSIVE : &str = "permissive";
            const DEFAULT_SECURITY_MODE : &str = SECURITY_MODE_ENFORCE;

            let security_mode = extension_config["security-mode"].as_str().unwrap_or(DEFAULT_SECURITY_MODE);
            if security_mode != SECURITY_MODE_ENFORCE && security_mode != SECURITY_MODE_PERMISSIVE {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid security mode")));
            }
            info!("security mode: {}", security_mode);

            if security_mode == SECURITY_MODE_ENFORCE {
                if response.verification_result == false {
                    let message = match response.message {
                        Some(message) => message,
                        None => "TLS verification failed".to_string(),
                    };
                    debug!("TLS extension method error: {}", message);
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, message)));
                }
            }
     
            let base64_cert = response.certificate.as_str();
            let pem = STANDARD.decode(base64_cert).map_err(|err| format!("Failed to decode aTLS certificate. Error: {}", err))?;
            let certificate = reqwest::Certificate::from_pem(&pem).map_err(|err| format!("Failed to parse aTLS certificate. Error: {}", err))?;
        
            certificates.push(certificate);
        }

        let mut client = ClientBuilder::new();
        for certificate in certificates {
            info!("Adding aTLS certificate as trusted to the HTTP client");
            client = client.add_root_certificate(certificate).danger_accept_invalid_certs(true);
        }

        Ok(Transport {
            config: config.clone(),
            client: client.build()?.clone(),
        })
    }

    /// Sends an HTTP request with the specified method, endpoint, data, and parameters.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method for the request.
    /// * `endpoint` - The endpoint for the request.
    /// * `data` - The data to send with the request (optional).
    /// * `params` - The query parameters for the request (optional).
    ///
    /// # Returns
    ///
    /// A `Result` containing the response body as a string, or an error if the request fails.
    async fn request(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        data: Option<Value>,
        params: Option<Value>,
    ) -> Result<String, Box<dyn Error>> {
        let endpoint = endpoint.trim_start_matches('/');
        let url = &self.config.base_path.join(endpoint).map_err(|e| {
            error!("Invalid endpoint (shouldn't contain base url): {}. Error: {}", endpoint, e);
            Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid endpoint")) as Box<dyn std::error::Error>
        })?;

        let mut request_builder = self.client.request(method, url.clone());

        if let Some(ref user_agent) = self.config.user_agent {
            request_builder = request_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        if let Some(ref bearer_token) = self.config.bearer_access_token {
            request_builder = request_builder.bearer_auth(bearer_token);
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

        debug!("Sending request: {:?}", request_builder);
        debug!("Request body: {:?}", data);
        debug!("Request params: {:?}", params);
        debug!("Request URL: {:?}", url.as_str());

        let resp = request_builder.send().await.map_err(|e| {
            log::error!("HTTP request failed: {}", e);
            e
        })?;

        let status = resp.status();
        let content = resp.text().await.map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Failed to read response text: {}", e))
        })?;

        debug!("Response status: {}", status);
        debug!("Response body: {}", content);
        if status.is_success() {
            Ok(content)
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Request failed with status: {}. Response: {}", status, content),
            )))
        }
    }

    /// Sends a GET request to the specified endpoint with the given query parameters.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint for the request.
    /// * `params` - The query parameters for the request (optional).
    ///
    /// # Returns
    ///
    /// A `Result` containing the response body as a string, or an error if the request fails.
    pub async fn get(
        &self,
        endpoint: &str,
        params: Option<Value>,
    ) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::GET, endpoint, None, params)
            .await
    }

    /// Sends a POST request to the specified endpoint with the given data.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint for the request.
    /// * `data` - The data to send with the request (optional).
    ///
    /// # Returns
    ///
    /// A `Result` containing the response body as a string, or an error if the request fails.
    pub async fn post(
        &self,
        endpoint: &str,
        data: Option<Value>,
    ) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::POST, endpoint, data, None)
            .await
    }
    
    /// Sends a PUT request to the specified endpoint with the given data.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint for the request.
    /// * `data` - The data to send with the request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response body as a string, or an error if the request fails.
    pub async fn put(&self, endpoint: &str, data: Value) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::PUT, endpoint, Some(data), None)
            .await
    }

    /// Sends a DELETE request to the specified endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint for the request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response body as a string, or an error if the request fails.
    pub async fn delete(&self, endpoint: &str) -> Result<String, Box<dyn Error>> {
        self.request(reqwest::Method::DELETE, endpoint, None, None)
            .await
    }

    // other HTTP methods can be added here
}

#[cfg(test)]
mod tests {
    use crate::utils::configuration::Configuration;
    use crate::utils::transport::Transport;
    use mockito::mock;
    use url::Url;

    // effectively no sense in testing various responses, as it's reqwest's responsibility
    // we should test Transport's methods instead

    #[tokio::test]
    async fn test_request() {
        let base_url_str = mockito::server_url();
        let base_url = Url::parse(&base_url_str).expect("Failed to parse mock server URL");
        
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message": "success"}"#)
            .create();

        let config = Configuration::new(base_url.clone());
        let transport = Transport::new(&config.clone());
        let response = transport.get("/test", None).await;

        assert!(response.is_ok());
        let body = response.unwrap();
        assert_eq!(body, r#"{"message": "success"}"#);
    }
}