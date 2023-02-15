use actix_web::HttpMessage;
use futures::TryFutureExt;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HOST, USER_AGENT};
use reqwest::Response;
use crate::error::MyError;

pub async fn get_user(access_token: &str) -> Result<String, MyError> {
    let client = reqwest::Client::new();
    let response = client.get("https://api.github.com/user")
        .header(ACCEPT, "application/vnd.github+json")
        .header(AUTHORIZATION, "Bearer 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")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header(USER_AGENT, "https://github.com/ptcdevs/rust-api")
        .send()
        .map_err(|err| MyError::GithubApi(err.to_string()))
        .await?;
    //TODO: throw MyError if status code is not 200
    let body = response
        .text()
        .map_err(|err| MyError::GithubApi(err.to_string()))
        .await?;
    //TODO: throw MyError if cannot parse out username

    println!("tba");
    todo!()
}
