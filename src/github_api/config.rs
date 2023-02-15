use std::collections::{HashMap, HashSet};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use crate::error::MyError;
use crate::error::MyError::{TokenResponseBodyError, TokenResponseError};

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

        let client = Self::parse_access_token(&response_body);

        client
    }
}

impl GithubConfig {
    pub fn parse_access_token(access_token_response: &str) -> Result<GithubClient, MyError> {
        // eg: access_token=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&scope=repo%2Cuser&token_type=bearer
        let query_string_elements = access_token_response
            .split("&")
            .map(|elems| {
                let mut elem_split = elems
                    .split("=");
                let key = elem_split
                    .next()
                    .ok_or_else(|| MyError::TokenResponseParseError)?;
                let value = elem_split
                    .next()
                    .ok_or(MyError::TokenResponseParseError)?;

                Ok((key, value))
            });
        let kv_pair: Result<Vec<(&str, &str)>, MyError> = query_string_elements
            .filter(|kv| kv.is_ok())
            .collect();
        let parsed_hashmap: HashMap<&str, &str> = HashMap::from_iter(kv_pair?);
        let parsed_keys: HashSet<&str> = parsed_hashmap
            .keys()
            .cloned()
            .collect();
        let expected_keys: HashSet<&str> = vec!["access_token", "scope", "token_type"]
            .into_iter()
            .collect();
        let token_type = parsed_hashmap["token_type"].to_string();
        let scope: Vec<String> = parsed_hashmap["scope"]
            .to_string()
            .split("%2C")
            .map(|spl| spl.to_string())
            .collect();
        let expected_scopes = vec!["user".to_string(), "repo".to_string()];

        let client_result = if parsed_keys == expected_keys
            && token_type.eq("bearer")
            && scope.iter().all(|s| expected_scopes.contains(s)) {
            let scopes = parsed_hashmap
                .get("scope")
                .ok_or(MyError::TokenResponseParseError)?
                .split("%2C")
                .map(|e| e.to_string())
                .collect();
            Ok(GithubClient {
                token: parsed_hashmap[&"access_token"].to_string(),
                token_type: parsed_hashmap[&"token_type"].to_string(),
                scopes,
            })
        } else {
            Err(MyError::TokenResponseParseError)
        };

        client_result
    }
}

#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    pub code: String,
    pub state: String,
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct GithubClient {
    pub token: String,
    pub scopes: Vec<String>,
    pub token_type: String,
}
