use std::borrow::Borrow;
use std::cmp;
use reqwest::Client;
use crate::github_api::client::client::GithubClient;

pub mod client {
    #[derive(Default, Debug)]
    pub struct GithubClient {
        pub token: String,
        pub scopes: String,
        pub token_type: String,
    }

    impl GithubClient {
        pub fn new(access_token_response: &str) {
            todo!()
        }
    }
}

