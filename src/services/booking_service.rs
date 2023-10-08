use crate::models::requests::booking_requests::BookingRequest;
use reqwest::Response;
use reqwest_middleware::{ClientWithMiddleware, Result};
use crate::base::api_client::base_url;
use crate::services::service::generate_service;

generate_service!(BookingService, "/booking");

impl<'a> BookingService<'a> {
    pub async fn get_bookings(&self) -> Result<Response> {
        self.client.get(&self.path).send().await
    }

    pub async fn post_bookings(&self, body: &BookingRequest) -> Result<Response> {
        self.client.post(&self.path).json(body).send().await
    }
}
