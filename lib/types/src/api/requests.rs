use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::impl_has_response;

/// GET VERSION
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersion {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
    pub version: String,
}

impl_has_response!(GetVersion, GetVersionResponse);

/// GET SERVER
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServer {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServerReponse {
    pub id: String,
}

impl_has_response!(GetServer, GetServerReponse);

// GET DEPLOYMENT
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDeployment {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDeploymentResponse {
    pub id: String,
}

impl_has_response!(GetDeployment, GetDeploymentResponse);

/// GET BUILD
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuild {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuildResponse {
    pub id: String,
}

impl_has_response!(GetBuild, GetBuildResponse);
