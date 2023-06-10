use types::api::{
    requests::{
        GetBuild, GetBuildResponse, GetDeployment, GetDeploymentResponse, GetServer,
        GetServerResponse, GetVersion, GetVersionResponse,
    },
    Request, Resolve,
};

use crate::state::AppState;

impl AppState {
    pub async fn handle_request(&self, request: Request) -> anyhow::Result<String> {
        match request {
            Request::GetVersion(req) => self.resolve_to_string(req).await,
            Request::GetServer(req) => self.resolve_to_string(req).await,
            Request::GetDeployment(req) => self.resolve_to_string(req).await,
            Request::GetBuild(req) => self.resolve_to_string(req).await,
        }
    }
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
