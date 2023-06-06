use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::HasResponse;

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
    fn req_type() -> &'static str {
        "GetVersion"
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServer {
    pub id: String,
}

impl HasResponse for GetServer {
    type Response = GetServer;
    fn req_type() -> &'static str {
        "GetServer"
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDeployment {
    pub id: String,
}

impl HasResponse for GetDeployment {
    type Response = GetDeployment;
    fn req_type() -> &'static str {
        "GetDeployment"
    }
}
