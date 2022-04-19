use reqwest::{Client, Error, Response};

#[derive(Debug)]
pub struct BitbucketClient {
    http_client: Client,
}

impl BitbucketClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    pub async fn get(&self, uri: &str) -> Result<Response, Error> {
        self.http_client.get(uri).send().await
    }
}
