use crate::base::api_client::base_url;
use crate::base::types::cookie_client::CookieClient;
use crate::models::requests::booking_requests::BookingRequest;
use crate::services::service::generate_service;
use reqwest::cookie::Jar;
use reqwest::Response;
use reqwest::Url;
use reqwest_middleware::{ClientWithMiddleware, Result};
use std::sync::Arc;

generate_service!(BookingService, "/booking");

impl<'a> BookingService<'a> {
    pub async fn get_bookings(&self) -> Result<Response> {
        self.client.get(&self.path).send().await
    }

    pub async fn post_bookings(&self, body: &BookingRequest) -> Result<Response> {
        self.client.post(&self.path).json(body).send().await
    }
}
