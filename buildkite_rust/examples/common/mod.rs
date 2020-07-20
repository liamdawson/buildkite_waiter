#![macro_use]

const TEST_TOKEN: &str = "test_token";
const EXPECTED_AUTH_HEADER: &str = "Bearer test_token";

pub fn json_snapshot_mock(method: &str, path: &str, name: &str, status: usize) -> mockito::Mock {
    let fixture_path = format!("{}/examples/fixtures/{}.json", env!("CARGO_MANIFEST_DIR"), name);

    mockito::mock(method, path)
        .match_header("Authorization", EXPECTED_AUTH_HEADER)
        .with_status(status)
        .with_body_from_file(&fixture_path)
}

fn mockito_url() -> reqwest::Url {
    reqwest::Url::parse(&mockito::server_url())
        .expect("Failed to parse mockito server_url")
}

pub fn subject() -> buildkite_rust::Buildkite {
    buildkite_rust::Buildkite::authenticated(TEST_TOKEN)
        .api_url(mockito_url())
}

#[macro_export]
/// Execute the supplied expression, and keep processing/transforming it until there's a valid response and body to return.
/// Panics with step/expression specific errors if an expectation isn't met.
macro_rules! expect_response_and_body {
    ($subject:expr) => {{
        let response = $subject
            .await.expect(&format!("Request triggered by {} failed", stringify!($subject)))
            .error_for_status().expect(&format!("Request triggered by {} returned non-success", stringify!($subject)));

        let body = response
            .body().expect(&format!("Request body parse for {}", stringify!($subject)))
            .clone();

        (response, body)
    }};
}
