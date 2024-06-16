

#[derive(Debug, Clone)]
pub struct ServiceConfiguration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}


impl ServiceConfiguration {
    pub fn new() -> ServiceConfiguration {
        ServiceConfiguration::default()
    }

    pub fn set_base_path(&mut self, base_path: &str) -> &mut Self {
        self.base_path = base_path.to_string();
        self
    }
}

impl Default for ServiceConfiguration {
    fn default() -> Self {
        ServiceConfiguration {
            base_path: "localhost".to_owned(),
            user_agent: Some("GA4GH SDK".to_owned()),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}