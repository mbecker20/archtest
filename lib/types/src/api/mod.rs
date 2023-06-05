use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod requests;

pub trait RequestResponse {
    type Request: Serialize + std::fmt::Debug;
    type Response: DeserializeOwned + std::fmt::Debug;
    fn req_type() -> &'static str;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Request {
    GetVersion,
    GetServer(Id),
    GetDeployment(Id),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Id {
    pub id: String,
}

impl Id {
    pub fn new(id: impl Into<String>) -> Id {
        Id { id: id.into() }
    }
}
