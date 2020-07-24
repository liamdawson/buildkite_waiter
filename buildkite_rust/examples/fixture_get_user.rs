mod common;
use common::*;

#[tokio::main]
async fn main() {
    let client = common::subject();

    let _mock = json_snapshot_mock("GET", "/user", "get_user", 200).create();

    let (_, body) = expect_response_and_body!(client.user().get_access_token_holder());

    println!("{:#?}", body);
}
