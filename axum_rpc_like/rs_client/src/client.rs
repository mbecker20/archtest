use anyhow::{anyhow, Context};
use reqwest::StatusCode;
use serde_json::json;
use types::api::HasResponse;

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

    pub async fn request<T: HasResponse>(
        &self,
        request: T,
    ) -> anyhow::Result<T::Response> {
        let req_type = T::req_type();
        trace!("sending request | type: {req_type} | body: {request:?}");
        let res = self
            .reqwest
            .post(&self.api_url)
            .json(&json!({
                "type": req_type,
                "params": request
            }))
            .send()
            .await?;
        let status = res.status();
        trace!("got response | type: {req_type} | {status} | body: {res:?}",);
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
