#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

// Check whether defining BasicAuth works like this or not, else revert to the basic definition commented out
#[derive(Debug, Clone)]
pub struct BasicAuth {
    pub username: String,
    pub password: Option<String>,
}
// pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
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

    pub fn set_base_path(&mut self, base_path: &str) -> &mut Self {
        self.base_path = base_path.to_string();
        self
    }
}

impl Default for Configuration {
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
