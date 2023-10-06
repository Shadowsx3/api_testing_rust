use crate::models::requests::booking_requests::BookingRequest;
use reqwest::Response;
use reqwest_middleware::{ClientWithMiddleware, Result};

pub struct BookingService<'a> {
    client: &'a ClientWithMiddleware,
    path: String,
}

impl<'a> BookingService<'a> {
    pub fn new(client: &'a ClientWithMiddleware, base_url: String) -> Self {
        let path = format!("{}{}", &base_url, "/booking");
        Self { client, path }
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub async fn get_bookings(&self) -> Result<Response> {
        self.client.get(&self.path).send().await
    }

    pub async fn post_bookings(&self, body: &BookingRequest) -> Result<Response> {
        self.client.post(&self.path).json(body).send().await
    }
}
