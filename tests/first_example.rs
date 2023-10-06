use api_testing_rust::client::*;
use reqwest::header::HeaderMap;
use reqwest_middleware::ClientWithMiddleware;
use rstest::*;
use std::collections::HashMap;
use api_testing_rust::responses::idk::Root;

#[rstest]
#[tokio::test]
async fn should_success(_client: &ClientWithMiddleware) {
    assert_eq!(42, 42)
}

#[rstest]
#[tokio::test]
async fn should_fail(client: &ClientWithMiddleware, base_url: &str, default_headers: HeaderMap) {
    let query = vec![("drilldowns", "Nation"), ("measures", "Population")];
    let result = client
        .get(base_url)
        .query(&query)
        .send()
        .await;
    if let Err(error) = result {
        panic!("{}", error)
    }
    let response = result.unwrap();

    assert_eq!(response.status(), 200);
    //println!("{}", response.text().await.unwrap());
    let json = response.json::<Root>().await.unwrap();
    println!("{:#?}", json);
}
