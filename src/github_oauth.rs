pub mod github_oauth {
    use actix_web::{get, Error, Responder, http::StatusCode, web::Redirect};
    use utoipa::{ToSchema};
    use std::{sync::Mutex, collections::HashMap};
    use std::os::linux::raw::stat;
    use once_cell::sync::Lazy;

    #[derive(Clone)]
    pub struct GithubOauthConfig {
        pub client_id: String,
        pub client_secret: String,
        pub redirect_url: String,
        pub scopes: Vec<String>,
    }

    impl GithubOauthConfig {
        pub fn get_authorize_url(&self) -> (String, String) {
            let state = "fo1Ooc1uofoozeithimah4iaW";
            let authorize_url = format!(
                "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}&state={}&allow_signup=false",
                self.client_id,
                self.redirect_url,
                self.scopes.join("+"),
                state
            );

            (authorize_url, state.to_string())
        }
    }

    pub struct GithubOauthConfigBorrowed<'a> {
        pub client_id: &'a str,
        pub client_secret: &'a str,
        pub redirect_url: &'a str,
        pub scopes: &'a Vec<String>,
    }

    impl<'a> GithubOauthConfigBorrowed<'a> {
        pub fn get_authorize_url(&self) -> (String, String) {
            let state = "fo1Ooc1uofoozeithimah4iaW";
            let authorize_url = format!(
                "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}&state={}&allow_signup=false",
                self.client_id,
                self.redirect_url,
                self.scopes.join("+"),
                state
            );

            (authorize_url, state.to_string())
        }
    }

}
