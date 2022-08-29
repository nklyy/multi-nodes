use std::collections::HashMap;

use reqwest::{Method, RequestBuilder};
use serde_json::Value;

pub type Headers = HashMap<String, String>;

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
        headers: Option<&Headers>,
        add_data: T,
    ) -> Result<reqwest::Response, reqwest::Error>
    where
        T: Fn(RequestBuilder) -> RequestBuilder,
    {
        let client = reqwest::Client::new();

        let mut request = client.request(method.clone(), url);

        // Setting the headers, if any
        if let Some(headers) = headers {
            let headers = headers.try_into().unwrap();

            request = request.headers(headers);
        }

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
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.request(Method::POST, url, headers, |req| req.json(payload))
            .await
    }
}
