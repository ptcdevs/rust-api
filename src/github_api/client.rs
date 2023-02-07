use std::borrow::Borrow;
use std::cmp;
use reqwest::Client;
use crate::github_api::client::client::GithubClient;

pub mod client {
    #[derive(Default, Debug)]
    pub struct GithubClient {
        pub client: reqwest::Client,
        pub access_token: String,
        pub scopes: String,
        pub token_type: String,
    }
}
