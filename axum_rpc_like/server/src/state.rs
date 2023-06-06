use std::{net::SocketAddr, str::FromStr, sync::Arc};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Env {
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Env {
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("0.0.0.0:{}", self.port)).unwrap()
    }
}

pub struct AppState {
    pub env: Env,
}

impl AppState {
    pub async fn load() -> Arc<AppState> {
        let env: Env = envy::from_env().unwrap();

        AppState { env }.into()
    }
}

fn default_port() -> u16 {
    11217
}
