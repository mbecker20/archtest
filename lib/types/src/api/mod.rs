use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use typeshare::typeshare;

use self::requests::{GetBuild, GetDeployment, GetServer, GetVersion};

pub mod requests;
#[async_trait::async_trait]
pub trait HasResponse: Serialize + DeserializeOwned + std::fmt::Debug + Send + 'static {
    type Response: Serialize + DeserializeOwned + std::fmt::Debug;
    fn req_type() -> &'static str;
}

#[async_trait::async_trait]
pub trait Resolve<Req: HasResponse> {
    async fn resolve(&self, req: Req) -> anyhow::Result<Req::Response>;
    async fn resolve_to_string(&self, req: Req) -> anyhow::Result<String> {
        let res = self.resolve(req).await?;
        let res = serde_json::to_string(&res).context("failed at serializing response")?;
        Ok(res)
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "params")]
pub enum Request {
    GetVersion(GetVersion),
    GetServer(GetServer),
    GetDeployment(GetDeployment),
    GetBuild(GetBuild),
}
