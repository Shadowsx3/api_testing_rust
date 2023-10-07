use api_testing_rust::base::api_client::{base_url, client};
use api_testing_rust::models::requests::booking_requests::BookingRequest;
use api_testing_rust::models::responses::booking_responses::{
    Booking, BookingAddResponse, BookingIdsResponse,
};
use api_testing_rust::services::booking_service::BookingService;
use pretty_assertions::assert_eq;
use reqwest_middleware::ClientWithMiddleware;
use rstest::*;
use speculoos::assert_that;
use speculoos::prelude::*;
use api_testing_rust::models::shared::booking::Bookingdates;

#[rstest]
#[tokio::test]
async fn should_get_booking(client: ClientWithMiddleware, base_url: &str) {
    let booking_service = BookingService::new(&client, base_url);
    let response = booking_service.get_bookings().await.unwrap();

    assert_eq!(response.status(), 200);

    let json_body = response.json::<BookingIdsResponse>().await.unwrap();

    assert_that!(&json_body.len()).is_greater_than(100);
}

#[rstest]
#[tokio::test]
async fn should_post_booking(client: ClientWithMiddleware, base_url: &str) {
    let booking_service = BookingService::new(&client, base_url);
    let response = booking_service
        .post_bookings(&BookingRequest {
            firstname: "Jim".to_string(),
            lastname: "Brown".to_string(),
            totalprice: 111,
            depositpaid: true,
            bookingdates: Bookingdates {
                checkin: "2018-01-01".to_string(),
                checkout: "2019-01-01".to_string(),
            },
            additionalneeds: "Breakfast".to_string(),
        })
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let json_body = response.json::<BookingAddResponse>().await.unwrap();
    let expected_body = Booking {
        firstname: "Jim".to_string(),
        lastname: "Brown".to_string(),
        totalprice: 111,
        depositpaid: true,
        bookingdates: Bookingdates {
            checkin: "2018-01-01".to_string(),
            checkout: "2019-01-01".to_string(),
        },
        additionalneeds: "Breakfast".to_string(),
    };

    assert_eq!(json_body.booking, expected_body);
}
