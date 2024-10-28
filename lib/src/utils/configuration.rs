use url::Url;
use serde_json::Value;
use crate::clients::ServiceType;
use log::warn;
/// A struct representing a configuration for the SDK.
///
/// The `Configuration` struct is responsible for specifying details of the Endpoint where the requests are made.
/// It provides methods for making constructing new configuration, changing the base url, and specifying a default configuration.
#[derive(Debug, Clone)]
pub struct Configuration {
    /// The base path for API requests.
    pub base_path: Url,
    /// The user agent to be used in API requests.
    pub user_agent: Option<String>,
    /// The basic authentication credentials.
    pub basic_auth: Option<BasicAuth>,
    /// The OAuth access token for authentication.
    pub oauth_access_token: Option<String>,
    /// The bearer access token for authentication.
    pub bearer_access_token: Option<String>,
    /// The API key for authentication.
    pub api_key: Option<ApiKey>,
}

/// Represents the basic authentication credentials.
#[derive(Debug, Clone, PartialEq)]
pub struct BasicAuth {
    /// The username for basic authentication.
    pub username: String,
    /// The password for basic authentication.
    pub password: Option<String>,
}

/// Represents the API key for authentication.
#[derive(Debug, Clone, PartialEq)]
pub struct ApiKey {
    /// The prefix for the API key.
    pub prefix: Option<String>,
    /// The key value of the API key.
    pub key: String,
}

impl Configuration {
    /// Creates a new instance of Configuration.
    ///
    /// # Arguments
    ///
    /// * `base_path` - The base path for API requests.
    /// * `user_agent` - The user agent to be used in API requests.
    /// * `basic_auth` - The basic authentication credentials.
    /// * `oauth_access_token` - The OAuth access token for authentication.
    ///
    /// # Returns
    ///
    /// A new instance of Configuration.
    pub fn new(
        base_path: Url,
    ) -> Self {
        Configuration {
            base_path,
            user_agent: Some("GA4GH SDK".to_owned()),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }

    /// Sets the base path for API requests.
    ///
    /// # Arguments
    ///
    /// * `base_path` - The base path for API requests.
    ///
    /// # Returns
    ///
    /// A mutable reference to the Configuration instance.
    pub fn set_base_path(&mut self, base_path: Url) -> &mut Self {
        self.base_path = base_path;
        self
    }
    
    /// Sets the user agent for API requests.
    ///
    /// # Arguments
    ///
    /// * `user_agent` - The user agent to be used in API requests.
    ///
    /// # Returns
    ///
    /// A new instance of Configuration.
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    /// Sets the basic authentication credentials for API requests.
    ///
    /// # Arguments
    ///
    /// * `basic_auth` - The basic authentication credentials.
    ///
    /// # Returns
    ///
    /// A new instance of Configuration.
    pub fn with_basic_auth(mut self, basic_auth: BasicAuth) -> Self {
        self.basic_auth = Some(basic_auth);
        self
    }

    /// Sets the OAuth access token for API requests.
    ///
    /// # Arguments
    ///
    /// * `oauth_access_token` - The OAuth access token for authentication.
    ///
    /// # Returns
    ///
    /// A new instance of Configuration.
    pub fn with_oauth_access_token(mut self, oauth_access_token: String) -> Self {
        self.oauth_access_token = Some(oauth_access_token);
        self
    }

    /// Loads the configuration from a JSON file.
    ///
    /// # Arguments
    ///
    /// * `service_type` - The type of service to load the configuration for.
    ///
    /// # Errors
    ///
    /// This function will return an error if the configuration file is missing or malformed.

    pub async fn from_file(service_type: ServiceType)-> Result<Self, Box<dyn std::error::Error>> {
        let config_file_path = dirs::home_dir().ok_or("Home directory not found")?.join(".ga4gh-cli/config.json");
        if config_file_path.exists() {
            let contents = std::fs::read_to_string(config_file_path)?;
            let config_json: Value = serde_json::from_str(&contents)?;
            if !config_json.is_object() {
                return Err("Configuration file must be a JSON object".into());
            }
            if !config_json[service_type.as_str()].is_object() {
                return Err("Configuration file must contain the requested `{service_type}` configuration".into());
            }
            let config_json = config_json[service_type.as_str()].as_object().unwrap();
            if !config_json["base_path"].is_string() {
                return Err("Configuration file must contain a 'base_path' string".into());
            }
            let base_path = Url::parse(config_json["base_path"].as_str().unwrap_or_default())?;
            let mut config = Configuration::new(base_path);
            if config_json["basic_auth"].is_object() {
                let basic_auth = BasicAuth {
                    username: config_json["basic_auth"]["username"].as_str().unwrap_or_default().to_string(),
                    password: Some(config_json["basic_auth"]["password"].as_str().unwrap_or_default().to_string()),
                };
                config = config.with_basic_auth(basic_auth);
            }
            if config_json["oauth_access_token"].is_string() {
                let oauth_access_token = config_json["oauth_access_token"].as_str().unwrap_or_default().to_string();
                config = config.with_oauth_access_token(oauth_access_token);
            }
            return Ok(config);
        }
        warn!("Configuration file not found at {:?}, using empy defualt configuration", config_file_path);
        Ok(Configuration::default())
    }
}

impl Default for Configuration {
    /// Creates a default instance of Configuration.
    ///
    /// # Returns
    ///
    /// A default instance of Configuration.
    /// This is used to define a configuration for a server that is running on your localhost
    fn default() -> Self {
        Configuration {
            base_path: Url::parse("http://localhost").unwrap(),
            user_agent: Some("GA4GH SDK".to_owned()),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_new_configuration() {
        let config = Configuration::new(
    Url::parse("https://api.example.com").unwrap(),
        )
        .with_user_agent("My User Agent".to_owned())
        .with_basic_auth(BasicAuth {
            username: "admin".to_owned(),
            password: Some("password".to_owned()),
        })
        .with_oauth_access_token("my_oauth_token".to_owned());

        assert_eq!(config.base_path.as_str(), "https://api.example.com/");
        assert_eq!(config.user_agent, Some("My User Agent".to_owned()));
        assert_eq!(
            config.basic_auth,
            Some(BasicAuth {
                username: "admin".to_owned(),
                password: Some("password".to_owned()),
            })
        );
        assert_eq!(config.oauth_access_token, Some("my_oauth_token".to_owned()));
        assert_eq!(config.bearer_access_token, None);
        assert_eq!(config.api_key, None);
    }

    #[test]
    fn test_set_base_path() {
        let mut config = Configuration::default();
        config.set_base_path(Url::parse("https://api.example.com").unwrap());
        assert_eq!(config.base_path.as_str(), "https://api.example.com/");
    }
}
