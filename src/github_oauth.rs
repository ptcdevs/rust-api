pub mod github_oauth {
    use actix_web::{get, Error, Responder, http::StatusCode, web::Redirect};
    use utoipa::{ToSchema};
    use std::{sync::Mutex, collections::HashMap};
    use once_cell::sync::Lazy;

    pub struct GithubOauthConfig {
        pub client_id: String,
        pub client_secret: String,
        pub redirect_url: String,
        pub scopes: String,
    }

    pub static GLOBAL_DATA: once_cell::sync::Lazy<Mutex<GithubOauthConfig>> = once_cell::sync::Lazy::new(|| {
        let mut config = GithubOauthConfig {
            client_id: "".to_string(),
            client_secret: "".to_string(),
            redirect_url: "".to_string(),
            scopes: "".to_string(),
        };

        Mutex::new(config)
    });

    pub fn get_authorize_url(state: String) -> String {
        "".to_string()
    }
}
