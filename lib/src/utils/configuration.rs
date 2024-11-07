use crate::clients::ServiceType;
use crate::utils::extension_manager::ExtensionManager;
use log::{debug};
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde::ser::SerializeStruct;
use serde_json::Value;
use std::io::Read;
use std::fs::File;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledExtension {
    pub name: String,
    pub version: String,
    pub path: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledExtensions {
    pub extensions: Vec<InstalledExtension>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ServiceExtensionConfiguration {
    #[serde(rename = "name")]
    pub extension_name: String,
    pub required: bool,
    pub configuration: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceExtensionsConfiguration(Vec<ServiceExtensionConfiguration>);

impl ServiceExtensionsConfiguration {
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
        //                 "extension specific-key": "value"
        //             }
        //         }
        //     }
        // }
       let mut config: Configuration = if !service_type.is_none() {
            let service_type = service_type.unwrap();
            debug!("Reading service configuration file: {}", service_config_path);
            let mut file = File::open(service_config_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let config_json: Value = serde_json::from_str(&contents)?;
            if !config_json[service_type.as_str()].is_object() {
                return Err(format!("Configuration file must contain the requested `{}` configuration", service_type).into());
            }
            let config_json = config_json[service_type.as_str()].clone();

            serde_json::from_value(config_json)?
        } else {
            Configuration::default()
        };

        // Example configuration JSON of the globally installed extensions
        // {
        //     "extensions": [
        //       {
        //         "name": "/full/path/to/extension-name.ga4gh-sdk-extension.json",
        //         "enabled": true
        //       }
        //     ]
        // }
        // Read the configuration file of the globally available extensions
        debug!("Reading extensions configuration file: {}", extensions_config_path);
        let mut file = File::open(extensions_config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config_json: Value = serde_json::from_str(&contents)?;
        if !config_json["extensions"].is_array() {
            return Err("Extensions configuration file must contain an array of extensions".into());
        }

        let installed_extensions: InstalledExtensions = serde_json::from_value(config_json)?;
        config.extensions_manager = ExtensionManager::new(installed_extensions.clone(), config.extensions.take())?;

        Ok(config)
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: Url::parse("https://localhost").unwrap(),
            user_agent: None,
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

