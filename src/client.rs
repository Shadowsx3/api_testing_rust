use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use rstest::*;
use std::time::Duration;
use crate::middleware::LoggingMiddleware;

#[fixture]
pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
    headers
}

#[fixture]
#[once]
pub fn client(default_headers: HeaderMap) -> ClientWithMiddleware {
    let new_client = Client::builder()
        .default_headers(default_headers)
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Error building client");

    ClientBuilder::new(new_client)
        .with(LoggingMiddleware)
        .build()
}

#[fixture]
pub fn base_url() -> &'static str {
    "https://datausa.io/api/data"
}
