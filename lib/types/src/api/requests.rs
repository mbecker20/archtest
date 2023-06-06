use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::HasResponse;
use crate::impl_has_req_type;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersion {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
    pub version: String,
}

impl HasResponse for GetVersion {
    type Response = GetVersionResponse;
}

impl_has_req_type!(GetVersion);

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServer {
    pub id: String,
}

impl HasResponse for GetServer {
    type Response = GetServer;
}

impl_has_req_type!(GetServer);

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDeployment {
    pub id: String,
}

impl HasResponse for GetDeployment {
    type Response = GetDeployment;
}

impl_has_req_type!(GetDeployment);
