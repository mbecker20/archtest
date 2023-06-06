use anyhow::Context;
use types::api::requests::GetVersionResponse;

use crate::state::AppState;

impl AppState {
    pub fn get_version() -> anyhow::Result<String> {
        serde_json::to_string(&GetVersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
        .context("failed to convert version to string")
    }
}
