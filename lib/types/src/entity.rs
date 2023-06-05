use bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Server {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Deployment {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    pub id: String,
    pub name: String,
    pub description: String,
    pub server_id: String,
    pub image: DeploymentImage
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum DeploymentImage {
    Build { id: String },
    Custom { image: String }
}

impl Default for DeploymentImage {
    fn default() -> DeploymentImage {
        DeploymentImage::Custom { image: Default::default() }
    }
}
