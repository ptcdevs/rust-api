use std::borrow::Borrow;
use std::cmp;
use reqwest::Client;
use crate::github_api::client::client::GithubClient;

pub mod client {
    use std::collections::HashMap;
    use futures::{StreamExt, TryFutureExt, TryStreamExt};
    use futures::stream::Collect;
    use crate::error::MyError;

    #[derive(Default, Debug)]
    pub struct GithubClient {
        pub token: String,
        pub scopes: String,
        pub token_type: String,
    }

    impl GithubClient {
        pub fn new(access_token_response: &str) -> Result<HashMap<&str, &str>, MyError> {
            // eg: access_token=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&scope=repo%2Cuser&token_type=bearer
            let elements: Vec<Result<(&str,&str),MyError>> = access_token_response
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
                })
                .collect();
            let kv_pair: Vec<(&str,&str)> = elements
                .clone()
                .into_iter()
                .filter(|kv| kv.is_ok())
                .map(|kv: Result<(&str,&str),MyError>| kv.unwrap())
                .collect();
                // .map_ok_or_else(|kv: Result<(&str,&str),MyError>| kv.unwrap(), MyError::TokenResponseParseError);

            let parsed: Result<HashMap<&str,&str>,MyError> = if kv_pair.clone().into_iter().count() == 3 {
                Ok(HashMap::from_iter(kv_pair))
            }  else {
                Err(MyError::TokenResponseParseError)
            };

            parsed
        }
    }
}

