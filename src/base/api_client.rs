use crate::base::middleware::LoggingMiddleware;
use crate::base::types::proxy_config::ProxyConfig;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT};
use reqwest::{Client, Proxy};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use rstest::*;
use std::time::Duration;
use log::{info};

fn setup_logger() {
    dotenv::dotenv().ok();
    let _ = env_logger::builder().format_target(false).is_test(true).try_init();
}

#[fixture]
pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers
}

#[fixture]
pub fn proxy() -> ProxyConfig {
    ProxyConfig::new()
}

#[fixture]
#[once]
pub fn client(default_headers: HeaderMap, proxy: ProxyConfig) -> ClientWithMiddleware {
    setup_logger();

    info!("Client created with: Headers: {:?}", default_headers.clone());
    let mut client_builder = Client::builder()
        .default_headers(default_headers)
        .cookie_store(true)
        .timeout(Duration::from_secs(60));

    if proxy.is_enabled() {
        info!("Proxy enabled");
        client_builder = client_builder
            .proxy(Proxy::all(proxy.get_url()).unwrap())
            .danger_accept_invalid_certs(true);
    }

    ClientBuilder::new(client_builder.build().expect("Error in the client builder"))
        .with(LoggingMiddleware)
        .build()
}

#[fixture]
pub fn base_url() -> &'static str {
    "https://restful-booker.herokuapp.com"
}
