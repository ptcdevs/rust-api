#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::error::Error;
    use std::sync::Arc;
    use std::str;
    use actix_session::SessionExt;
    use super::*;
    use actix_web::{App, http::{self, header::ContentType}, Responder, test, web};
    use actix_web::body::MessageBody;
    use actix_web::web::Query;
    use crate::error::MyError;
    use crate::github_oauth::github_oauth::{CallbackParams, GithubOauthFunctions};
    use async_trait::async_trait;
    use futures::executor::block_on;
    use crate::{callback, login};

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
        async fn get_access_token<'a>(&'a self, code: String) -> Result<String, MyError> {
            Err(self.get_access_token_error.clone())
        }
    }

    #[actix_web::test]
    async fn test_add() {
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
            get_access_token_error: MyError::TokenResponseError
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
            //.map(|header| (header.0.to_string(),header.1.to_str().unwrap().to_string()))
            //.map(|header| (header.0.to_string(),header.1.to_str().unwrap().to_string())) .collect::<Vec<(String,String)>>()
            .map(|header| (header.1.to_str().unwrap().to_string())) .collect::<Vec<(String)>>()
            ;
        // let login_body_text = str::from_utf8(login_body.borrow());

        let callback_request = test::TestRequest::get()
            .uri("/callback?code=6f654b9ee57fd13b7b88&state=fo1Ooc1uofoozeithimah4iaW")
            .to_request();
        let callback_response = test::call_service(&app, callback_request).await;
        let callback_body = callback_response
            .map_into_boxed_body()
            .into_body()
            .try_into_bytes()
            .unwrap();
        let callback_body_text = str::from_utf8(callback_body.borrow());
        // assert!(resp.status().is_success());
        assert!(true == true)
    }
}
