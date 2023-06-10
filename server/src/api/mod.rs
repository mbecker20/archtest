use derive::Resolver;
use serde::{Deserialize, Serialize};
use types::api::{
    requests::{
        GetBuild, GetBuildResponse, GetDeployment, GetDeploymentResponse, GetServer,
        GetServerResponse, GetVersion, GetVersionResponse, Other, OtherResponse,
    },
    Resolve,
};
use typeshare::typeshare;

use crate::state::AppState;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[serde(tag = "type", content = "params")]
#[resolver_target(AppState)]
pub enum Request {
    GetVersion(GetVersion),
    GetServer(GetServer),
    GetDeployment(GetDeployment),
    GetBuild(GetBuild),
    Other(Other),
}

#[async_trait::async_trait]
impl Resolve<GetVersion> for AppState {
    async fn resolve(&self, _: GetVersion) -> anyhow::Result<GetVersionResponse> {
        Ok(GetVersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}

#[async_trait::async_trait]
impl Resolve<GetServer> for AppState {
    async fn resolve(&self, req: GetServer) -> anyhow::Result<GetServerResponse> {
        Ok(GetServerResponse { id: req.id })
    }
}

#[async_trait::async_trait]
impl Resolve<GetDeployment> for AppState {
    async fn resolve(&self, req: GetDeployment) -> anyhow::Result<GetDeploymentResponse> {
        Ok(GetDeploymentResponse { id: req.id })
    }
}

#[async_trait::async_trait]
impl Resolve<GetBuild> for AppState {
    async fn resolve(&self, req: GetBuild) -> anyhow::Result<GetBuildResponse> {
        Ok(GetBuildResponse { id: req.id })
    }
}

#[async_trait::async_trait]
impl Resolve<Other> for AppState {
    async fn resolve(&self, _: Other) -> anyhow::Result<OtherResponse> {
        Ok(OtherResponse {
            other: String::from("other"),
        })
    }
}
