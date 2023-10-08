use crate::base::api_client::base_url;
use crate::base::types::cookie_client::CookieClient;
use crate::models::requests::auth_requests::AuthRequest;
use crate::models::responses::auth_responses::AuthResponse;
use crate::services::service::generate_service;
use reqwest::cookie::{CookieStore, Jar};
use reqwest::{Response, Url};
use reqwest_middleware::{ClientWithMiddleware, Result};
use std::sync::Arc;

pub struct AuthToken {
    pub token: String,
}

generate_service!(AuthService, "/auth", AuthToken);

impl<'a> AuthService<'a> {
    pub async fn get_auth(&self, credentials: AuthRequest) -> Result<Response> {
        self.client.post(&self.path).json(&credentials).send().await
    }

    pub async fn set_token(&mut self, auth_response: &AuthResponse) {
        self.set_data(AuthToken {
            token: auth_response.token.clone(),
        });
        self.cookie_jar.set_cookies(
            &mut [format!("token={}", auth_response.token).parse().unwrap()].iter(),
            &self.get_base_url(),
        );
    }

    pub async fn auth(&mut self, credentials: AuthRequest) {
        let response = self
            .client
            .post(&self.path)
            .json(&credentials)
            .send()
            .await
            .unwrap();
        let json_body = response.json::<AuthResponse>().await.unwrap();
        self.set_token(&json_body).await;
    }

    pub fn get_token(&self) -> Option<String> {
        if let Some(auth_token) = self.data.as_ref() {
            return Some(auth_token.token.clone());
        }
        None
    }
}
