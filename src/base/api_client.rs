use crate::base::middleware::LoggingMiddleware;
use crate::base::types::proxy_config::ProxyConfig;
use log::Level::Info;
use log::{info, log_enabled};
use reqwest::cookie::Jar;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::{Client, Proxy};
use reqwest_middleware::ClientBuilder;
use rstest::*;
use std::sync::{Arc, Once};
use std::time::Duration;
use crate::base::types::cookie_client::CookieClient;

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
pub fn client(default_headers: &HeaderMap, proxy: &ProxyConfig) -> CookieClient {
    setup_logger();

    info!(
        "Client created with: Headers: {:?}",
        default_headers.clone()
    );
    let jar = Arc::new(Jar::default());

    let mut client_builder = Client::builder()
        .default_headers(default_headers.clone())
        .cookie_provider(Arc::clone(&jar))
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

    CookieClient {client: client_builder.build(), cookie_jar: jar}
}

#[fixture]
pub fn base_url() -> &'static str {
    "https://restful-booker.herokuapp.com"
}
