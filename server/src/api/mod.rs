use types::api::{
    requests::{GetBuild, GetDeployment, GetServer},
    Request,
};

use crate::state::AppState;

mod get_version;

impl AppState {
    pub async fn handle_request(&self, request: Request) -> anyhow::Result<String> {
        match request {
            Request::GetVersion => AppState::get_version(),

            // Server
            Request::GetServer(req) => self.get_server(req),

            // Deployment
            Request::GetDeployment(req) => self.get_deployment(req),

            Request::GetBuild(req) => self.get_build(req),
        }
    }
}

impl AppState {

    pub fn get_server(&self, req: GetServer) -> anyhow::Result<String> {
        Ok(serde_json::to_string(&req)?)
    }

    pub fn get_deployment(&self, req: GetDeployment) -> anyhow::Result<String> {
        Ok(serde_json::to_string(&req)?)
    }

    pub fn get_build(&self, req: GetBuild) -> anyhow::Result<String> {
        Ok(serde_json::to_string(&req)?)
    }
}
