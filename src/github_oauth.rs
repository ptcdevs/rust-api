pub mod github_oauth {
    use actix_web::{get, Error, Responder, http::StatusCode, web::Redirect};
    use utoipa::{ToSchema};
    use std::{sync::Mutex, collections::HashMap};
    use once_cell::sync::Lazy;

    pub struct GithubOauthConfig {
        pub client_id: String,
        pub client_secret: String,
        pub redirect_url: String,
        pub scopes: Vec<String>,
    }

    impl GithubOauthConfig {
        pub fn new(client_id: String, client_secret: String, redirect_url: String, scopes: Vec<String>) -> GithubOauthConfig {
            GithubOauthConfig {
                client_id: client_id,
                client_secret: client_secret,
                redirect_url: redirect_url,
                scopes: scopes,
            }
        }

        pub fn get_authorize_url(&self) -> String {
            let authorizeUrl = "https://github.com/login/oauth/authorize?client_id=92e48c903bf0b3e8c4f3&redirect_uri=http%3A%2F%2Flocalhost%3A8080%2Fauth%2Fcallback&scope=user+repo&state=fo1Ooc1uofoozeithimah4iaW&allow_signup=false".to_string();
            "".to_string()
        }
    }
}
