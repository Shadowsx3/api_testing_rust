use api_testing_rust::base::api_client::client;
use api_testing_rust::base::types::cookie_client::CookieClient;
use api_testing_rust::models::requests::auth_requests::AuthRequest;
use api_testing_rust::models::responses::auth_responses::{AuthErrorResponse, AuthResponse};
use api_testing_rust::services::auth_service::AuthService;
use pretty_assertions::assert_eq;
use reqwest::cookie::CookieStore;
use rstest::*;

#[rstest]
#[tokio::test]
async fn should_auth_successfully(client: CookieClient) {
    let mut auth_service = AuthService::new(&client);
    let response = auth_service
        .get_auth(AuthRequest {
            username: "admin".to_string(),
            password: "password123".to_string(),
        })
        .await;
    assert_eq!(response.is_ok(), true);
    let json_body = response.unwrap().json::<AuthResponse>().await.unwrap();

    assert_eq!(json_body.token.is_empty(), false);
    auth_service.set_token(&json_body).await;
    let cookies = client
        .cookie_jar
        .cookies(&auth_service.get_base_url())
        .unwrap();
    assert_eq!(cookies.len(), 21);
    assert_eq!(cookies, format!("token={}", json_body.token));
}

#[rstest]
#[tokio::test]
async fn should_trow_auth_error(client: CookieClient) {
    let auth_service = AuthService::new(&client);
    let response = auth_service
        .get_auth(AuthRequest {
            username: "admin".to_string(),
            password: "password".to_string(),
        })
        .await;
    assert_eq!(response.is_ok(), true);
    let json_error = response.unwrap().json::<AuthErrorResponse>().await.unwrap();

    assert_eq!(json_error.reason, "Bad credentials");
}

#[rstest]
#[tokio::test]
async fn should_auth_successfully_and_set_cookies(client: CookieClient) {
    let mut auth_service = AuthService::new(&client);
    auth_service
        .auth(AuthRequest {
            username: "admin".to_string(),
            password: "password123".to_string(),
        })
        .await;
    let cookies = client
        .cookie_jar
        .cookies(&auth_service.get_base_url())
        .unwrap();
    assert_eq!(cookies.len(), 21);
    assert_eq!(cookies, format!("token={}", auth_service.get_token().unwrap()));
}
