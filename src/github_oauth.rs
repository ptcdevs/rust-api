
pub mod github_oauth {
    use actix_web::{Error, get, http::StatusCode, Responder, web::Redirect};
    use utoipa::ToSchema;
    use std::{collections::HashMap, sync::Mutex};
    use std::os::linux::raw::stat;
    use once_cell::sync::Lazy;
    use crate::error::MyError;
    use crate::error::MyError::{EmptyTokenError, MissingStateError, SessionError, TokenRequestError};
    use reqwest::Client;
    use serde::Deserialize;
    use urlencoding::decode;

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
                state);

            (authorize_url, state.to_string())
        }

        // POST https://github.com/login/oauth/access_token
        // Content-Type: application/x-www-form-urlencoded
        //
        // client_id=92e48c903bf0b3e8c4f3&client_secret={{GITHUB_OAUTH_CLIENT_SECRET}}&code=3ce16bbcfff7a152295d&redirect_uri=http://localhost:8080/auth/callback
        pub async fn get_access_token(&self, code: String) -> Result<String, MyError> {
            let token_url = "https://github.com/login/oauth/access_token";
            let token_request_body = format!(
                "client_id={}&client_secret={}&code={}&redirect_uri={}",
                self.client_id,
                self.client_secret,
                code,
                self.redirect_url);

            let client = reqwest::Client::new();
            let response = client.post(token_url)
                .body(token_request_body)
                .send()
                .await
                .map_err(|err| TokenRequestError)?;
            let body = response
                .text()
                .await
                .map_err(|err| EmptyTokenError);

            body
        }
    }

    pub struct GithubOauthConfigBorrowed<'a> {
        pub client_id: &'a str,
        pub client_secret: &'a str,
        pub redirect_url: &'a str,
        pub scopes: &'a Vec<String>,
    }

    impl<a> GithubOauthConfigBorrowed<'a> {
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

    #[derive(Debug, Deserialize)]
    pub struct CallbackParams {
        pub code: String,
        pub state: String,
    }

    //access_token=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&scope=repo%2Cuser&token_type=bearer
    #[derive(Debug, Deserialize)]
    pub struct AccessTokenResponse {
        pub access_token: String,
        pub scope: String,
        pub token_type: String
    }

    impl AccessTokenResponse {
        pub fn get_scopes(&self) -> Result<Vec<String>,Error> {
            urlencoding::decode(self.scope.to_string())
        }
    }
}

