pub mod config {
    use async_trait::async_trait;
    use reqwest::Client;
    use serde::Deserialize;

    use crate::error::MyError;
    use crate::error::MyError::{TokenResponseBodyError, TokenResponseError};
    use crate::github_api::client::client::GithubClient;

    #[derive(Clone)]
    pub struct GithubConfig {
        pub client_id: String,
        pub client_secret: String,
        pub redirect_url: String,
        pub scopes: Vec<String>,
    }

    #[derive(Clone)]
    pub struct GithubOauthClient {
        pub client_id: String,
        pub client_secret: String,
        pub redirect_url: String,
        pub scopes: Vec<String>,
    }

    #[async_trait]
    pub trait GithubOauthFunctions {
        fn get_authorize_url(&self) -> (String, String);
        async fn get_client(&self, code: &str) -> Result<GithubClient, MyError>;
    }

    #[async_trait]
    impl GithubOauthFunctions for GithubConfig {
        fn get_authorize_url(&self) -> (String, String) {
            let state = "fo1Ooc1uofoozeithimah4iaW";
            let authorize_url = format!(
                "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}&state={}&allow_signup=false",
                self.client_id,
                self.redirect_url,
                self.scopes.join("+"),
                state);

            (authorize_url, state.to_string())
        }
        async fn get_client(&self, code: &str) -> Result<GithubClient, MyError> {
            let token_url = "https://github.com/login/oauth/access_token";
            let token_request_body = format!(
                "client_id={}&client_secret={}&code={}&redirect_uri={}",
                self.client_id,
                self.client_secret,
                code,
                self.redirect_url);

            let response = Client::new()
                .post(token_url)
                .body(token_request_body)
                .send()
                .await
                .map_err(|err| TokenResponseError)?;

            //TODO: validate response, make sure scopes match
            let response_status = response
                .status()
                .is_success();

            let response_body = response
                .text()
                .await
                .map_err(|err| TokenResponseBodyError)?;

            let client = GithubClient::new(&response_body);

            client
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct CallbackParams {
        pub code: String,
        pub state: String,
    }
}
