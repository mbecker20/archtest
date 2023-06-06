use serde::{de::DeserializeOwned, Deserialize, Serialize};
use typeshare::typeshare;

pub mod requests;

pub trait RequestResponse {
    type Request: Serialize + std::fmt::Debug;
    type Response: DeserializeOwned + std::fmt::Debug;
    fn req_type() -> &'static str;
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "params")]
pub enum Request {
    GetVersion,
    GetServer(Id),
    GetDeployment(Id),
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Id {
    pub id: String,
}

impl Id {
    pub fn new(id: impl Into<String>) -> Id {
        Id { id: id.into() }
    }
}
