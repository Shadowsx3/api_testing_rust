use serde_derive::{Deserialize, Serialize};
use crate::models::shared::booking::Bookingdates;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingRequest {
    pub firstname: String,
    pub lastname: String,
    pub totalprice: i64,
    pub depositpaid: bool,
    pub bookingdates: Bookingdates,
    pub additionalneeds: String,
}