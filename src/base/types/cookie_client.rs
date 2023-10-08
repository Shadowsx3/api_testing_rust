use std::sync::Arc;
use reqwest::cookie::Jar;
use reqwest_middleware::ClientWithMiddleware;

pub struct CookieClient {
    pub client: ClientWithMiddleware,
    pub cookie_jar: Arc<Jar>,
}