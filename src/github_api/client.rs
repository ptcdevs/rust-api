use std::borrow::Borrow;
use std::cmp;
use reqwest::Client;
use crate::github_api::client::client::GithubClient;

pub mod client {
    use std::collections::{HashMap, HashSet};
    use futures::{StreamExt, TryFutureExt, TryStreamExt};
    use futures::stream::Collect;
    use crate::error::MyError;

    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct GithubClient<'a> {
        pub token: &'a str,
        pub scopes: Vec<&'a str>,
        pub token_type: &'a str,
    }

    impl <'a> GithubClient<'a> {
        pub fn new(access_token_response: &'a str) -> Result<GithubClient<'a>,MyError> {
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

                    Ok((key,value))
                });
            let kv_pair: Result<Vec<(&str,&str)>,MyError> = query_string_elements
                .filter(|kv| kv.is_ok())
                .collect();
            let parsed_hashmap: HashMap<&str,&str> = HashMap::from_iter(kv_pair?);
            let parsed_keys: HashSet<&str> = parsed_hashmap
                .keys()
                .cloned()
                .collect();
            let expected_keys: HashSet<&str> = vec!["access_token", "scope", "token_type"]
                .into_iter()
                .collect();

            let client_result = if parsed_keys == expected_keys {
                let scopes: Vec<&'a str> = parsed_hashmap
                    .get("scope")
                    .ok_or(MyError::TokenResponseParseError)?
                    .split("%2C")
                    .map(|e| e)
                    .collect();
                Ok(GithubClient {
                    token: parsed_hashmap[&"access_token"],
                    token_type: parsed_hashmap[&"token_type"],
                    scopes,
                })
            } else {
                Err(MyError::TokenResponseParseError)
            };

            client_result
        }


    }
}

