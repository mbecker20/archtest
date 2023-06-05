use serde::{Deserialize, Serialize};

use super::{Id, RequestResponse};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersion {
    pub version: String,
}

impl RequestResponse for GetVersion {
    type Request = ();
    type Response = GetVersion;
    fn req_type() -> &'static str {
        "GetVersion"
    }
}

pub struct GetServer;

impl RequestResponse for GetServer {
    type Request = Id;
    type Response = Id;
    fn req_type() -> &'static str {
        "GetServer"
    }
}

pub struct GetDeployment;

impl RequestResponse for GetDeployment {
    type Request = Id;
    type Response = Id;
    fn req_type() -> &'static str {
        "GetDeployment"
    }
}
