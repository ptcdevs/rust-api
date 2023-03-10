pub mod config {
    use actix_web::{Error, get, http::StatusCode, Responder, web::Redirect};
    use utoipa::ToSchema;
    use std::{collections::HashMap, sync::Mutex};
    use std::borrow::{Borrow, Cow};
    use std::os::linux::raw::stat;
    use std::string::FromUtf8Error;
    use once_cell::sync::Lazy;
    use crate::error::MyError;
    use crate::error::MyError::{EmptyTokenError, MissingStateError, SessionError, TokenResponseError, TokenResponseBodyError};
    use reqwest::{Client, Response};
    use serde::Deserialize;
    use urlencoding::decode;
    use async_trait::async_trait;
    use crate::github_api::client::client::GithubClient;
    use url::Url;


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
        async fn get_client<'a>(&'a self, code: String) -> Result<GithubClient, MyError>;
        fn parse_client<'a>(&'a self, access_token_text: &str) -> Result<GithubClient, Error>;
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
        async fn get_client<'a>(&'a self, code: String) -> Result<GithubClient, MyError> {
            let token_url = "https://github.com/login/oauth/access_token";
            let token_request_body = format!(
                "client_id={}&client_secret={}&code={}&redirect_uri={}",
                self.client_id,
                self.client_secret,
                code,
                self.redirect_url);

            //TODO: extract this to trait function for testing
            let response = Client::new()
                .post(token_url)
                .body(token_request_body)
                .send()
                .await
                .map_err(|err| TokenResponseError)?;
            let response_status = response
                .status()
                .is_success();
            let response_body = response
                .text()
                .await
                .map_err(|err| TokenResponseBodyError)?;

            // Ok(GithubClient{
            //     access_token: "".to_string(),
            //     scopes: "".to_string(),
            //     token_type: "".to_string(),
            // })
            todo!()
        }
        fn parse_client<'a>(&'a self, access_token_text: &str) -> Result<GithubClient, Error> {
            todo!()
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct CallbackParams {
        pub code: String,
        pub state: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct AccessTokenResponse<'a> {
        pub access_token: &'a str,
        pub scope: &'a str,
        pub token_type: &'a str,
    }

    impl<'a> AccessTokenResponse<'a> {
        pub fn get_scopes(&'a self) -> Result<Cow<'a, str>, FromUtf8Error> {
            let decoded_scopes = urlencoding::decode(self.scope.borrow());
            decoded_scopes
        }
    }
}
