use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// GET VERSION
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetVersionResponse)]
pub struct GetVersion {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
    pub version: String,
}

/// GET SERVER
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetServerResponse)]
pub struct GetServer {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServerResponse {
    pub id: String,
}

// GET DEPLOYMENT
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetDeploymentResponse)]
pub struct GetDeployment {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDeploymentResponse {
    pub id: String,
}

/// GET BUILD
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetBuildResponse)]
pub struct GetBuild {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuildResponse {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(OtherResponse)]
pub struct Other {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OtherResponse {
    pub other: String,
}
