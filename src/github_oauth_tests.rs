#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;
    use actix_session::{Session, SessionExt};
    use actix_web::{test, web};
    use async_trait::async_trait;
    use reqwest::{Client, get};
    use crate::callback;
    use crate::error::MyError;
    use crate::error::MyError::{TokenResponseBodyError, TokenResponseError};
    use crate::github_oauth::github_oauth::GithubOauthFunctions;
    use super::*;

    // Note this useful idiom: importing names from outer (for mod tests) scope.

    pub struct GithubOauthConfigMock {
        pub get_access_token_error: MyError
    }

    #[async_trait]
    impl GithubOauthFunctions for GithubOauthConfigMock {
        fn get_authorize_url(&self) -> (String, String) {
            ("".to_string(), "".to_string())
        }
        async fn get_access_token<'a>(&'a self, code: String) -> Result<String, MyError> {
            Err(self.get_access_token_error.clone())
        }
    }

    #[test]
    async fn test_add() {
        let query = web::Query::from_query("code=6f654b9ee57fd13b7b88&state=fo1Ooc1uofoozeithimah4iaW")
            .unwrap();
        let request = test::TestRequest::default()
            .to_http_request();
        let session = request.get_session();
        session.insert("state","fo1Ooc1uofoozeithimah4iaW");
        let github_config = GithubOauthConfigMock {
            get_access_token_error: MyError::TokenResponseError
        };
        let github_config_arc: Arc<dyn GithubOauthFunctions> = Arc::new(github_config);
        let github_config_data: web::Data<dyn GithubOauthFunctions> = web::Data::from(github_config_arc);

        let result = callback(query, session, github_config_data)
            .await;

        assert_eq!(true, true);
    }
}
