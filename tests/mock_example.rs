use api_testing_rust::base::api_client::{base_url, client};
use api_testing_rust::models::responses::idk::*;
use reqwest_middleware::ClientWithMiddleware;
use rstest::*;
use speculoos::assert_that;
use speculoos::prelude::*;

#[rstest]
#[tokio::test]
async fn should_fail(client: &ClientWithMiddleware, base_url: &str) {
    let query = vec![("drilldowns", "Nation"), ("measures", "Population")];
    let result = client.get(base_url).query(&query).send().await;
    if let Err(error) = result {
        panic!("{}", error)
    }
    let response = result.unwrap();

    assert_eq!(response.status(), 200);
    let json = response.json::<Root>().await.unwrap();
    assert_that(&json.data).has_length(8);
    assert_that!(json.data[0].nation.as_ref()).is_equal_to("United States");
    assert_that!(&json.data).contains(&Daum {
        id_nation: "01000US".to_string(),
        nation: "United States".to_string(),
        id_year: 2019,
        year: "2019".to_string(),
        population: 324697795,
        slug_nation: "united-states".to_string(),
    });
}
