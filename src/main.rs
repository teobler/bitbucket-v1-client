use bitbucket_v1_client::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = client::BitbucketClient::new();

    let resp = client.get("https://www.rust-lang.org").await?;
    println!("{:#?}", resp);
    Ok(())
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::{MockServer, GET};
    use serde_json::json;

    #[derive(serde::Deserialize, Debug)]
    struct TestStruct {
        test: String,
    }

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let server = MockServer::start();
        let test_mock = server.mock(|when, then| {
            when.method(GET).path("/test");
            then.status(200)
                .header("content-type", "text/html")
                .json_body(json!({ "test": "ohi" }));
        });

        let response = reqwest::blocking::get(server.url("/test"))?.json::<TestStruct>()?;

        test_mock.assert();
        assert_eq!(response.test, "ohi");
        Ok(())
    }
}
