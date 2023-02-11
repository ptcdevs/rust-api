pub mod client {
    use std::collections::{HashMap, HashSet};
    pub use futures::StreamExt;
    use crate::error::MyError;

    #[derive(Default, Debug, PartialEq, Eq, Clone)]
    pub struct GithubClient {
        pub token: String,
        pub scopes: Vec<String>,
        pub token_type: String,
    }

    impl GithubClient {
        pub fn new(access_token_response: &str) -> Result<GithubClient, MyError> {
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
}

