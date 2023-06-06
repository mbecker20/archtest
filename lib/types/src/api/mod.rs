use serde::{de::DeserializeOwned, Deserialize, Serialize};
use typeshare::typeshare;

use self::requests::{GetDeployment, GetServer};

pub mod requests;

pub trait HasResponse: Serialize + std::fmt::Debug {
    type Response: DeserializeOwned + std::fmt::Debug;
    fn req_type() -> &'static str;
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "params")]
pub enum Request {
    GetVersion,
    GetServer(GetServer),
    GetDeployment(GetDeployment),
}

// #[typeshare]
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Id {
//     pub id: String,
// }

// impl Id {
//     pub fn new(id: impl Into<String>) -> Id {
//         Id { id: id.into() }
//     }
// }
