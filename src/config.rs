use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AppConfig {
     pub(crate) github_oauth: GithubOauth,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct GithubOauth {
    app_name: String,
    app_url: String,
    pub(crate) client_id: String,
    pub(crate) redirect_url: String,
    pub(crate) scopes: Vec<String>,
}
