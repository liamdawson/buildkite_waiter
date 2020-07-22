mod common;
use common::*;

#[tokio::main]
async fn main() {
    let client = common::subject();

    let _mock = json_snapshot_mock(
        "GET",
        "/organizations/my-great-org/pipelines/my-pipeline/builds?per_page=30",
        "get_pipeline_builds",
        200,
    )
    .create();

    let (_, body) = expect_response_and_body!(client
        .builds()
        .pipeline("my-great-org", "my-pipeline")
        .get());

    println!("{:#?}", body);
}
