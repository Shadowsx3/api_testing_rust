use dotenv::var;
pub struct ProxyConfig {
    base_url: String,
    port: String,
    enabled: bool,
}

impl ProxyConfig {
    pub(crate) fn new() -> Self {
        let default_host = "localhost";
        let default_port = "8080";
        let base_url = var("PROXY_BASE_URL").unwrap_or(default_host.to_string());
        let port = var("PROXY_PORT").unwrap_or(default_port.to_string());
        let enabled = var("PROXY_ENABLED")
            .map(|value| value == "true")
            .unwrap_or(false);

        ProxyConfig {
            base_url,
            port,
            enabled,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_url(&self) -> String {
        format!("{}:{}", self.base_url, self.port)
    }
}
