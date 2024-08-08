/// Represents the configuration for the SDK.
#[derive(Debug, Clone)]
pub struct Configuration {
    /// The base path for API requests.
    pub base_path: String,
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
#[derive(Debug, Clone)]
pub struct BasicAuth {
    /// The username for basic authentication.
    pub username: String,
    /// The password for basic authentication.
    pub password: Option<String>,
}

/// Represents the API key for authentication.
#[derive(Debug, Clone)]
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
        base_path: String,
        user_agent: Option<String>,
        basic_auth: Option<BasicAuth>,
        oauth_access_token: Option<String>,
    ) -> Self {
        Configuration {
            base_path,
            user_agent,
            basic_auth,
            oauth_access_token,
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
    pub fn set_base_path(&mut self, base_path: &str) -> &mut Self {        
        self.base_path = base_path.to_string();
        self
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
            base_path: "localhost".to_owned(),
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

    #[test]
    fn test_new_configuration() {
        let config = Configuration::new(
            "https://api.example.com".to_owned(),
            Some("My User Agent".to_owned()),
            Some(BasicAuth {
                username: "admin".to_owned(),
                password: Some("password".to_owned()),
            }),
            Some("my_oauth_token".to_owned()),
        );

        assert_eq!(config.base_path, "https://api.example.com");
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
        config.set_base_path("https://api.example.com");

        assert_eq!(config.base_path, "https://api.example.com");
    }
}
