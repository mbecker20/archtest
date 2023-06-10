#[macro_use]
extern crate log;

use client::Client;
use serde::Deserialize;
use simple_logger::SimpleLogger;
use types::api::requests;

mod client;

#[derive(Deserialize)]
struct Env {
    api_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .with_colors(true)
        .with_utc_timestamps()
        .init()?;
    let env: Env = envy::from_env()?;

    info!("client version: {}", env!("CARGO_PKG_VERSION"));

    let client = Client::new(env.api_url);

    let version = client.request(requests::GetVersion {}).await?.version;

    info!("server version: {version}");

    let server = client
        .request(requests::GetServer {
            id: "server_id".into(),
        })
        .await?;

    info!("server: {server:?}");

    let deployment = client
        .request(requests::GetDeployment { id: "shit".into() })
        .await?;

    info!("deployment: {deployment:?}");

    let other = client.request(requests::Other {}).await?;

    info!("other: {other:?}");

    Ok(())
}
