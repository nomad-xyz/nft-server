use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use ethers::types::U256;
use tokio::task::JoinHandle;

use crate::MetadataGenerator;

async fn nft_handler<T>(Path(token_id): Path<U256>, State(generator): State<Arc<T>>) -> Response
where
    T: MetadataGenerator,
{
    match generator.metadata_for(token_id).await {
        Some(metadata) => (
            [("Cache-Control", "max-age=300, must-revalidate")],
            Json(metadata),
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "unknown token id").into_response(),
    }
}

async fn contract_handler<T>(State(generator): State<Arc<T>>) -> Response
where
    T: MetadataGenerator,
{
    match generator.contract_metadata().await {
        Some(metadata) => (
            [("Cache-Control", "max-age=300, must-revalidate")],
            Json(metadata),
        )
            .into_response(),
        None => (StatusCode::NOT_FOUND, "no contract metadata").into_response(),
    }
}

async fn return_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "unknown route")
}

pub fn serve<T>(t: T, socket: impl Into<SocketAddr>) -> JoinHandle<()>
where
    T: MetadataGenerator + Send + Sync + 'static,
{
    let addr = socket.into();

    tokio::spawn(async move {
        let app = Router::<_, Body>::with_state(Arc::new(t))
            .route("/:token_id", get(nft_handler))
            .route("/", get(contract_handler))
            .fallback(return_404);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    })
}
