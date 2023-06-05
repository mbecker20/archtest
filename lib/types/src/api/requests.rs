use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{Id, RequestResponse};

#[typeshare]
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

#[typeshare]
pub struct GetServer {}

impl RequestResponse for GetServer {
    type Request = Id;
    type Response = Id;
    fn req_type() -> &'static str {
        "GetServer"
    }
}

#[typeshare]
pub struct GetDeployment {}

impl RequestResponse for GetDeployment {
    type Request = Id;
    type Response = Id;
    fn req_type() -> &'static str {
        "GetDeployment"
    }
}
