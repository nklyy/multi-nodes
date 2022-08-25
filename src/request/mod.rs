use reqwest::{Method, RequestBuilder};
use serde_json::Value;

#[derive(Default, Debug, Clone)]
pub struct RequestClient;

impl RequestClient {
    pub fn new() -> RequestClient {
        RequestClient
    }

    async fn request<T>(
        &self,
        method: Method,
        url: &str,
        add_data: T,
    ) -> Result<reqwest::Response, reqwest::Error>
    where
        T: Fn(RequestBuilder) -> RequestBuilder,
    {
        let client = reqwest::Client::new();

        let mut request = client.request(method.clone(), url);

        request = request.header("Content-Type", "application/json");
        request = request.header("Accept", "application/json");

        request = add_data(request);

        // log::info!("Making request {:?}", request);
        let response = request.send().await?;

        Ok(response)
    }
}

impl RequestClient {
    // pub async fn get(&self, url: &str, payload: &Query) -> Result<String, RequestError> {
    //     self.request(Method::GET, url, headers, |req| req.query(payload))
    //         .await
    // }

    pub async fn post(
        &self,
        url: &str,
        payload: &Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.request(Method::POST, url, |req| req.json(payload))
            .await
    }
}
