use crate::base::api_client::base_url;
use crate::base::types::cookie_client::CookieClient;
use crate::models::requests::auth_requests::AuthRequest;
use crate::models::responses::auth_responses::AuthResponse;
use crate::services::service::generate_service;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::{Response, Url};
use reqwest_middleware::{ClientWithMiddleware, Result};
use std::sync::Arc;

generate_service!(AuthService, "/auth");

impl<'a> AuthService<'a> {
    pub async fn auth(&self, credentials: AuthRequest) -> Result<Response> {
        self.client.post(&self.path).json(&credentials).send().await
    }

    pub async fn set_token(&self, auth_response: &AuthResponse) {
        self.cookie_jar.set_cookies(
            &mut [format!("token={}", auth_response.token).parse().unwrap()].iter(),
            &self.get_base_url(),
        );
    }
}
