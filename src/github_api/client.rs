use std::borrow::Borrow;
use std::cmp;
use reqwest::Client;
use crate::github_api::client::client::GithubClient;

pub mod client {
    use std::collections::HashMap;
    use crate::error::MyError;

    #[derive(Default, Debug)]
    pub struct GithubClient {
        pub token: String,
        pub scopes: String,
        pub token_type: String,
    }

    impl GithubClient {
        pub fn new(access_token_response: &str) {
            let elements: Vec<Result<(String,String),MyError>> = access_token_response
                .split("&")
                .map(|elems| {
                    let mut elem_split = elems
                        .split("=");
                    let key = elem_split
                        .next()
                        .ok_or_else(|| MyError::TokenResponseParseError)?
                        .to_string();
                    let value = elem_split
                        .next()
                        .ok_or(MyError::TokenResponseParseError)?
                        .to_string();

                    Ok((key,value))
                })
                .collect();
            let kv_pair = elements
                .into_iter()
                .filter(|kv| kv.is_ok())
                .map(|kv| kv.unwrap());
            let parsed: HashMap<String,String> = HashMap::from_iter(kv_pair);
            todo!()
        }
    }
}

