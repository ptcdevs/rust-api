#[cfg(test)]
mod tests {
    use actix_web::body::MessageBody;
    use actix_web::web::Query;
    use actix_web::{App, test, web};
    use async_trait::async_trait;
    use crate::error::MyError;
    use std::borrow::Borrow;
    use std::str;
    use std::sync::Arc;

    use crate::{callback, login};
    use crate::github_api::client::client::GithubClient;
    use crate::github_api::config::config::{CallbackParams, GithubOauthFunctions};

    // Note this useful idiom: importing names from outer (for mod tests) scope.

    struct GithubOauthConfigMock {
        pub get_authorize_url_state: String,
        pub get_authorize_url_redirect_url: String,
        pub get_access_token_error: MyError,
    }

    #[async_trait]
    impl GithubOauthFunctions for GithubOauthConfigMock {
        fn get_authorize_url(&self) -> (String, String) {
            (self.get_authorize_url_redirect_url.to_string(), self.get_authorize_url_state.to_string())
        }
        async fn get_client(&self, _: String) -> Result<GithubClient, MyError> {
            Err(self.get_access_token_error.clone())
        }
    }

    #[actix_web::test]
    async fn test_github_login() {
        let query: Query<CallbackParams> = web::Query::from_query("code=6f654b9ee57fd13b7b88&state=fo1Ooc1uofoozeithimah4iaW")
            .unwrap();

        let state = "Ohbiuqu5di";
        let redirect_url = format!("https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}&state={}&allow_signup=false",
                                   "12345",
                                   "callback_url",
                                   "scopes",
                                   state
        );
        let github_config = GithubOauthConfigMock {
            get_authorize_url_state: state.to_string(),
            get_authorize_url_redirect_url: redirect_url.to_string(),
            get_access_token_error: MyError::TokenResponseError,
        };
        let github_config_arc: Arc<dyn GithubOauthFunctions> = Arc::new(github_config);
        let github_config_data: web::Data<dyn GithubOauthFunctions> = web::Data::from(github_config_arc);
        let app = test::init_service(
            App::new()
                .service(login)
                .service(callback)
                .app_data(github_config_data))
            .await;

        let login_request = test::TestRequest::get()
            .uri("/login")
            .to_request();
        let login_response = test::call_service(&app, login_request).await;
        let redirect_url = login_response
            .headers()
            .into_iter()
            .map(|header|
                (header.1.to_str().unwrap().to_string())).collect::<Vec<String>>();

        let callback_request = test::TestRequest::get()
            .uri(format!("/callback?code=6f654b9ee57fd13b7b88&state={}", state).as_str())
            .to_request();
        let callback_response = test::call_service(&app, callback_request).await;
        let callback_body = callback_response
            .map_into_boxed_body()
            .into_body()
            .try_into_bytes()
            .unwrap();
        let callback_body_text = str::from_utf8(callback_body.borrow());
        //TODO: and pass to GithubClient::new()
        // let client = GithubClient::new(callback_body_text);
        //TODO: update this to
        todo!()
    }

    #[actix_web::test]
    async fn parse_response_to_client() {
        let access_token = "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";
        let scopes = vec![
            "repo".to_string(),
            "user".to_string(),
        ];
        let token_type = "bearer";
        let access_token_response = format!("access_token={}&scope={}&token_type={}",
                                            access_token,
                                            scopes.join("%2C"),
                                            token_type,
        );

        let client = GithubClient::new(&access_token_response)
            .unwrap();
        let expected_client = GithubClient {
            token: access_token.to_string(),
            scopes: scopes,
            token_type: token_type.to_string(),
        };

        assert_eq!(client, expected_client, "GithubClient not being properly created from api response text");
    }
}
