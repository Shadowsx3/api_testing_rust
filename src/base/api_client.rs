use crate::base::middleware::LoggingMiddleware;
use crate::base::types::proxy_config::ProxyConfig;
use log::Level::Info;
use log::{info, log_enabled};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::{Client, Proxy};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use rstest::*;
use std::sync::Once;
use std::time::Duration;

static START: Once = Once::new();

fn setup_logger() {
    START.call_once(|| {
        dotenv::dotenv().ok();
        let _ = env_logger::builder()
            .format_target(false)
            .is_test(true)
            .try_init();
    });
}

#[fixture]
#[once]
pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers
}

#[fixture]
#[once]
pub fn proxy() -> ProxyConfig {
    ProxyConfig::new()
}

#[fixture]
pub fn client(default_headers: &HeaderMap, proxy: &ProxyConfig) -> ClientWithMiddleware {
    setup_logger();

    info!(
        "Client created with: Headers: {:?}",
        default_headers.clone()
    );
    let mut client_builder = Client::builder()
        .default_headers(default_headers.clone())
        .cookie_store(true)
        .timeout(Duration::from_secs(60));

    if proxy.is_enabled() {
        info!("Proxy enabled");
        client_builder = client_builder
            .proxy(Proxy::all(proxy.get_url()).unwrap())
            .danger_accept_invalid_certs(true);
    }

    let mut client_builder =
        ClientBuilder::new(client_builder.build().expect("Error in the client builder"));

    if log_enabled!(Info) {
        client_builder = client_builder.with(LoggingMiddleware);
    }

    client_builder.build()
}

#[fixture]
pub fn base_url() -> &'static str {
    "https://restful-booker.herokuapp.com"
}
