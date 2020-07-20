mod common;
use common::*;

#[tokio::main]
async fn main() {
    let client = common::subject();

    let _mock = json_snapshot_mock("GET", "/organizations/my-great-org/pipelines/my-pipeline/builds/1", "get_build", 200).create();

    let (_, body ) = expect_response_and_body!(client.build().get("my-great-org", "my-pipeline", 1));

    println!("{:#?}", body);
}
