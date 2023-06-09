#[macro_use]
extern crate log;

use std::sync::Arc;

use axum::{
    extract::State, headers::ContentType, http::StatusCode, routing::post, Json, Router,
    TypedHeader,
};
use simple_logger::SimpleLogger;
use state::AppState;

use crate::api::Request;

mod api;
mod helpers;
mod state;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .with_colors(true)
        .with_utc_timestamps()
        .init()
        .expect("failed to configure logger");

    info!("version: {}", env!("CARGO_PKG_VERSION"));

    let state = AppState::load().await;

    let socket_addr = state.env.socket_addr();

    let app = Router::new()
        .route(
            "/api",
            post(
                |state: State<Arc<AppState>>, Json(request): Json<Request>| async move {
                    info!("got request: {:?}", request);
                    let res = state
                        .resolve_request(request)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")));
                    if let Err(e) = &res {
                        info!("request error: {e:?}");
                    }
                    Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res?))
                },
            ),
        )
        .with_state(state);

    info!("starting server on {}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("server crashed")
}
