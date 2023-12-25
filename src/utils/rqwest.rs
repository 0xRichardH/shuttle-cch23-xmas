use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Error};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

pub struct RequestClient {
    client: ClientWithMiddleware,
}

impl RequestClient {
    pub fn new(max_reties: u32) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(max_reties);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self { client }
    }

    pub async fn get(&self, url: &str) -> Result<reqwest::Response, Error> {
        self.client.get(url).send().await
    }
}
