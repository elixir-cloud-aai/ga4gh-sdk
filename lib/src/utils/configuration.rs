//! This module defines the configuration structures and implementations for service extensions.
//!
//! It includes the following:
//! 
//! - `ServiceExtensionConfiguration`: Represents the configuration for a single service extension.
//! - `ServiceExtensionsConfiguration`: A collection of service extension configurations.
//! - Methods to retrieve and manage these configurations.

use crate::clients::ServiceType;
use crate::utils::extension_manager::ExtensionManager;
use log::{debug, error};
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;
use serde_json::Value;
use std::io::Read;
use std::fs::File;
use url::Url;

/// Represents the configuration for a single service extension.
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ServiceExtensionConfiguration {
    /// The name of the extension.
    #[serde(rename = "name")]
    pub extension_name: String,
    
    /// Indicates whether the extension is required.
    pub required: bool,
    
    /// The configuration details for the extension.
    pub configuration: Value,
}

/// A collection of service extension configurations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceExtensionsConfiguration(Vec<ServiceExtensionConfiguration>);

impl ServiceExtensionsConfiguration {
    /// Retrieves the configuration for a specific extension by name.
    ///
    /// # Arguments
    ///
    /// * `name` - A reference to a string that holds the name of the extension.
    ///
    /// # Returns
    ///
    /// * `&Value` - A reference to the configuration value of the specified extension.
    ///
    /// # Panics
    ///
    /// This function will panic if the extension with the specified name is not found.
    ///
    /// # Example
    ///
    /// ```rust
    /// let service_extensions_config = ServiceExtensionsConfiguration(vec![
    ///     ServiceExtensionConfig {
    ///         extension_name: "AttestedTLS-middleware".to_string(),
    ///         configuration: serde_json::json!({"trusted-repository": "trs://example-registry.org/"}),
    ///     },
    /// ]);
    /// // Considering that "AttestedTLS-middleware" exports "tls-verifier" extension method
    /// let config = service_extensions_config.get_extension_config(&"tls-verifier".to_string());
    /// println!("Extension configuration: {:?}", config);
    /// ```
    pub fn get_extension_config(&self, name: &String) -> &Value {
        &self.0.iter().find(|service_extension_config| service_extension_config.extension_name == *name).unwrap().configuration
    }
}

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
    /// service-related configuration for the extensions
    pub extensions: Option<ServiceExtensionsConfiguration>,
    /// The ExtensionManager instance for managing extensions
    pub extensions_manager: ExtensionManager,
}

impl Serialize for Configuration {
    /// Serializes the `Configuration` struct into a format suitable for storage or transmission.
    ///
    /// # Arguments
    ///
    /// * `serializer` - The serializer to use for converting the `Configuration` struct into a serializable format.
    ///
    /// # Returns
    ///
    /// * `Result<S::Ok, S::Error>` - The result of the serialization process.
    ///
    /// # Example
    ///
    /// ```rust
    /// let config = Configuration {
    ///     base_path: Url::parse("https://example.com").unwrap(),
    ///     user_agent: Some("MyApp/1.0".to_string()),
    ///     basic_auth: None,
    ///     oauth_access_token: None,
    ///     bearer_access_token: None,
    ///     api_key: None,
    ///     extensions: None,
    /// };
    /// let serialized = serde_json::to_string(&config).unwrap();
    /// println!("Serialized configuration: {}", serialized);
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Configuration", 8)?;
        state.serialize_field("base_path", &self.base_path.as_str())?;
        state.serialize_field("user_agent", &self.user_agent)?;
        state.serialize_field("basic_auth", &self.basic_auth)?;
        state.serialize_field("oauth_access_token", &self.oauth_access_token)?;
        state.serialize_field("bearer_access_token", &self.bearer_access_token)?;
        state.serialize_field("api_key", &self.api_key)?;
        state.serialize_field("extensions", &self.extensions)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Configuration {
    /// Deserializes the `Configuration` struct from a format suitable for storage or transmission.
    ///
    /// # Arguments
    ///
    /// * `deserializer` - The deserializer to use for converting the serialized data into a `Configuration` struct.
    ///
    /// # Returns
    ///
    /// * `Result<Self, D::Error>` - The result of the deserialization process.
    ///
    /// # Example
    ///
    /// ```rust
    /// let serialized = r#"
    /// {
    ///     "base_path": "https://example.com",
    ///     "user_agent": "MyApp/1.0",
    ///     "basic_auth": null,
    ///     "oauth_access_token": null,
    ///     "bearer_access_token": null,
    ///     "api_key": null,
    ///     "extensions": null
    /// }
    /// "#;
    /// let config: Configuration = serde_json::from_str(serialized).unwrap();
    /// println!("Deserialized configuration: {:?}", config);
    /// ```
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ConfigurationHelper {
            base_path: String,
            user_agent: Option<String>,
            basic_auth: Option<BasicAuth>,
            oauth_access_token: Option<String>,
            bearer_access_token: Option<String>,
            api_key: Option<ApiKey>,
            extensions: Option<ServiceExtensionsConfiguration>,
        }

        let helper = ConfigurationHelper::deserialize(deserializer)?;

        Ok(Configuration {
            base_path: Url::parse(&helper.base_path).map_err(serde::de::Error::custom)?,
            user_agent: helper.user_agent,
            basic_auth: helper.basic_auth,
            oauth_access_token: helper.oauth_access_token,
            bearer_access_token: helper.bearer_access_token,
            api_key: helper.api_key,
            extensions: helper.extensions,
            extensions_manager: ExtensionManager::default(),
        })
    }
}

/// Represents the basic authentication credentials.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicAuth {
    /// The username for basic authentication.
    pub username: String,
    /// The password for basic authentication.
    pub password: Option<String>,
}

/// Represents the API key for authentication.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiKey {
    /// The prefix for the API key.
    pub prefix: Option<String>,
    /// The key value of the API key.
    pub key: String,
}

impl Configuration {

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

    /// Loads configurations from JSON files and initializes extensions with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `service_type` - The type of service to load the configuration for.
    /// * `service_config_path` - The path to the service configuration file.
    /// * `extensions_config_path` - The path to the extensions configuration file.
    ///
    /// # Errors
    ///
    /// This function will return an error if the configuration file is missing or malformed.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Example service configuration JSON
    /// // {
    /// //     "TES": {
    /// //         "base_path": "https://some-host.org/ga4gh/tes/",
    /// //         "oauth_access_token": "...",
    /// //         "extensions": {
    /// //             "name": "extension-name",
    /// //             "required": true,
    /// //             "configuration": {
    /// //                 "extension specific key": "value"
    /// //             }
    /// //         }
    /// //     }
    /// // }
    /// let config = Configuration::from_file(
    ///     Some(ServiceType::TES),
    ///     &"path/to/service-config.json".to_string(),
    ///     &"path/to/extensions-config.json".to_string()
    /// )?;
    /// println!("Loaded configuration: {:?}", config);
    /// ```
    pub fn from_file(service_type: Option<ServiceType>, service_config_path: &String, extensions_config_path: &String) -> Result<Self, Box<dyn std::error::Error>> {
        // Example service configuration JSON
        // {
        //     "TES": {
        //         "base_path": "https://some-host.org/ga4gh/tes/",
        //         "oauth_access_token": "...",
        //         "extensions": {
        //             "name": "extension-name",
        //             "required": true,
        //             "configuration": {
        //                 "extension specific key": "value"
        //             }
        //         }
        //     }
        // }
        let mut config: Configuration = if let Some(service_type) = service_type {
            debug!("Reading service configuration file: {}", service_config_path);
            let file = File::open(service_config_path);
            let mut contents = String::new();
        
            match file {
                Ok(mut file) => {
                    if let Err(e) = file.read_to_string(&mut contents) {
                        error!("Failed to read configuration file: {}", e);
                        return Err(e.into());
                    }
                }
                Err(_) => {
                    debug!("Configuration file not found, using default configuration.");
                    return Ok(Configuration::default());
                }
            }
        
            let config_json: Value = serde_json::from_str(&contents)?;
            if !config_json[service_type.as_str()].is_object() {
                return Err(format!("Configuration file must contain the requested `{}` configuration", service_type).into());
            }
            let config_json = config_json[service_type.as_str()].clone();
        
            serde_json::from_value(config_json)?
        } else {
            Configuration::default()
        };

        config.extensions_manager = ExtensionManager::init(extensions_config_path, config.extensions.clone())?;

        Ok(config)
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
            base_path: Url::parse("https://localhost").unwrap(),
            user_agent: Some("GA4GH-SDK/CLI".to_owned()),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
            extensions: None,
            extensions_manager: ExtensionManager::default(),
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

