use httpmock::MockServer;
use rstest::*;

#[fixture]
pub fn mock_server() -> MockServer {
    MockServer::start()
}
