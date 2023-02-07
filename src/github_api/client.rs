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
            // eg: access_token=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&scope=repo%2Cuser&token_type=bearer
            let elements = access_token_response
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

                    Ok((key,value))
                });
            let kv_pair = elements
                .filter(|kv| kv.is_ok())
                .map(|kv: Result<(&str,&str),MyError>| kv.unwrap());
            let parsed: HashMap<&str,&str> = HashMap::from_iter(kv_pair);
            todo!()
        }
    }
}

