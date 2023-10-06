use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::models::shared::booking::Bookingdates;

pub type BookingIdsResponse = Vec<BookingIdItem>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingIdItem {
    pub bookingid: i64,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingAddResponse {
    pub bookingid: i64,
    pub booking: Booking,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Booking {
    pub firstname: String,
    pub lastname: String,
    pub totalprice: i64,
    pub depositpaid: bool,
    pub bookingdates: Bookingdates,
    pub additionalneeds: String,
}
