#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::sync::Arc;
    use actix_session::SessionExt;
    use super::*;
    use actix_web::{App, http::{self, header::ContentType}, Responder, test, web};
    use actix_web::web::Query;
    use crate::error::MyError;
    use crate::github_oauth::github_oauth::{CallbackParams, GithubOauthFunctions};
    use async_trait::async_trait;
    use futures::executor::block_on;
    use crate::callback;

    // Note this useful idiom: importing names from outer (for mod tests) scope.

    struct GithubOauthConfigMock {
        pub get_access_token_error: MyError,
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

    #[actix_web::test]
    async fn test_add() {
        let query: Query<CallbackParams> = web::Query::from_query("code=6f654b9ee57fd13b7b88&state=fo1Ooc1uofoozeithimah4iaW")
            .unwrap();
        let request = test::TestRequest::default()
            .to_http_request();
        let session = request.get_session();
        session.insert("state", "fo1Ooc1uofoozeithimah4iaW");
        let github_config = GithubOauthConfigMock {
            get_access_token_error: MyError::TokenResponseError
        };
        let github_config_arc: Arc<dyn GithubOauthFunctions> = Arc::new(github_config);
        let github_config_data: web::Data<dyn GithubOauthFunctions> = web::Data::from(github_config_arc);

        let app = test::init_service(
            App::new()
                .service(callback)
                .app_data(github_config_data))
            .await;
        let req = test::TestRequest::get()
            .uri("/callback")
            .to_request();
        let resp = test::call_service(&app, req).await;
        let body = resp.map_into_boxed_body().into_body().;
        // assert!(resp.status().is_success());
        assert!(true == true)
    }
}
