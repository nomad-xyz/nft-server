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
use tracing::{info_span, Instrument, Span};

use crate::MetadataGenerator;

/// Simple handler that consults the metadata generator, and returns EITHER the
/// token metadata, or a 404
pub async fn nft_handler<T>(
    Path(token_id): Path<String>,
    State(generator): State<Arc<T>>,
) -> Response
where
    T: MetadataGenerator,
{
    let e_500 = (
        StatusCode::INTERNAL_SERVER_ERROR,
        "service temporarily unavailable",
    );
    let token_id = match U256::from_dec_str(&token_id) {
        Ok(id) => id,
        Err(e) => {
            tracing::error!(token_id = ?token_id, error = %e, "error in token_id parsing");
            return e_500.into_response();
        }
    };

    match generator.metadata_for(token_id).await {
        Ok(Some(metadata)) => (
            [("Cache-Control", "max-age=300, must-revalidate")],
            Json(metadata),
        )
            .into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            [("Cache-Control", "max-age=300, must-revalidate")],
            "unknown token id",
        )
            .into_response(),
        Err(e) => {
            tracing::error!(error = %e, "error in metadata lookup");
            e_500.into_response()
        }
    }
}

/// Simple handler that consults the metadata generator, and returns EITHER the
/// contract metadata, or a 404
pub async fn contract_handler<T>(State(generator): State<Arc<T>>) -> Response
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

/// Fallback handler that returns a 404 with body `"unknown route"`
pub async fn return_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "unknown route")
}

/// Handler for healthcheck
pub async fn return_200() -> impl IntoResponse {
    StatusCode::OK
}

/// Serve an NFT generator at the specified socket address, running in a
/// provided span.
///
/// Adds routes for `/:token_id` and `/`, as well as a fallback 404. This is a
/// simple, works-out-of-the-box JSON metadata server with no additional app or
/// routing customization. If you would like to add additional routes, consider
/// defining the axum `Router` and handlers separately, and passing your router
/// to `serve_router`
pub fn serve_generator_with_span<T>(
    t: T,
    socket: impl Into<SocketAddr>,
    span: Span,
) -> JoinHandle<()>
where
    T: MetadataGenerator + Send + Sync + 'static,
{
    let app = Router::<_, Body>::with_state(Arc::new(t))
        .route("/healthcheck", get(return_200))
        .route(
            "/favicon.ico",
            get(|| async move { (StatusCode::NOT_FOUND, "") }),
        )
        .route("/:token_id", get(nft_handler))
        .route("/", get(contract_handler))
        .fallback(return_404);

    serve_router_with_span(app, socket, span)
}

/// Serve an NFT generator at the specified socket address.
///
/// Adds routes for `/:token_id` and `/`, as well as a fallback 404. This is a
/// simple, works-out-of-the-box JSON metadata server with no additional app or
/// routing customization. If you would like to add additional routes, consider
/// defining the axum `Router` and handlers separately, and passing your router
/// to `serve_router`
pub fn serve_generator<T>(t: T, socket: impl Into<SocketAddr>) -> JoinHandle<()>
where
    T: MetadataGenerator + Send + Sync + 'static,
{
    let span = info_span!("serve_generator");
    serve_generator_with_span(t, socket, span)
}

/// Serve an app with some shared state at the specified socket address
/// instrumented with the provided span.
///
/// Intended to allow full customization of the router. If a simple
/// no-customization JSON metadata server is required, instead use
pub fn serve_router_with_span<T>(
    app: Router<Arc<T>>,
    socket: impl Into<SocketAddr>,
    span: Span,
) -> JoinHandle<()>
where
    T: MetadataGenerator + Send + Sync + 'static,
{
    let addr = socket.into();
    tokio::spawn(async move {
        Instrument::instrument(
            axum::Server::bind(&addr).serve(app.into_make_service()),
            span,
        )
        .await
        .unwrap();
    })
}

/// Serve an app with some shared state at the specified socket address.
/// Intended to allow full customization of the router. If a simple
/// no-customization JSON metadata server is required, instead use
/// [`serve_generator`].
pub fn serve_router<T>(app: Router<Arc<T>>, socket: impl Into<SocketAddr>) -> JoinHandle<()>
where
    T: MetadataGenerator + Send + Sync + 'static,
{
    let span = info_span!("serve_router");
    serve_router_with_span(app, socket, span)
}
