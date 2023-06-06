use types::api::{requests::GetVersion, Request};

use crate::state::AppState;

impl AppState {
    pub async fn handle_request(&self, request: Request) -> anyhow::Result<String> {
        match request {
            Request::GetVersion => Ok(serde_json::to_string(&GetVersion {
                version: env!("CARGO_PKG_VERSION").to_string(),
            })?),
            Request::GetServer(id) => Ok(serde_json::to_string(&id)?),
            Request::GetDeployment(id) => Ok(serde_json::to_string(&id)?),
        }
    }
}