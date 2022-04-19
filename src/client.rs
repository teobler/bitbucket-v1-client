use reqwest::{Client, Error, Response};

#[derive(Debug)]
pub struct BitbucketClient {
    http_client: Client,
    bearer_auth: Option<String>,
}

impl BitbucketClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
            bearer_auth: None,
        }
    }

    pub fn with_auth(auth: String) -> Self {
        Self {
            http_client: Client::new(),
            bearer_auth: Some(auth),
        }
    }

    pub async fn get(&self, uri: &str) -> Result<Response, Error> {
        if let Some(ref auth) = self.bearer_auth {
            self.http_client.get(uri).bearer_auth(auth).send().await
        } else {
            self.http_client.get(uri).send().await
        }
    }
}
