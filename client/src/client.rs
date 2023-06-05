use anyhow::{anyhow, Context};
use reqwest::StatusCode;
use types::api::RequestResponse;

pub struct Client {
    reqwest: reqwest::Client,
    api_url: String,
}

impl Client {
    pub fn new(api_url: impl Into<String>) -> Client {
        Client {
            api_url: api_url.into(),
            reqwest: Default::default(),
        }
    }

    pub async fn request<T: RequestResponse>(
        &self,
        request: T::Request,
    ) -> anyhow::Result<T::Response> {
        trace!(
            "sending request | type: {} | body: {request:?}",
            T::req_type()
        );
        let res = self
            .reqwest
            .post(&self.api_url)
            .json(&request)
            .send()
            .await?;
        let status = res.status();
        trace!(
            "got response | type: {} | {status} | body: {res:?}",
            T::req_type()
        );
        if status == StatusCode::OK {
            res.json().await.context("failed to parse response to json")
        } else {
            let text = res
                .text()
                .await
                .context("failed to convert response to text")?;
            Err(anyhow!("request failed | {status} | {text}"))
        }
    }
}
