pub mod docs;
pub mod errors;
pub mod handlers;
pub mod router;

use std::net::SocketAddr;

use anyhow::Context;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use deadpool_diesel::postgres::Pool;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use pragma_common::services::{Service, ServiceRunner};

use docs::ApiDoc;
use router::api_router;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}

pub struct ApiService {
    state: AppState,
    host: String,
    port: u16,
}

impl ApiService {
    pub fn new(state: AppState, host: &str, port: u16) -> Self {
        Self {
            state,
            host: host.to_owned(),
            port,
        }
    }
}

#[async_trait::async_trait]
impl Service for ApiService {
    async fn start<'a>(&mut self, mut runner: ServiceRunner<'a>) -> anyhow::Result<()> {
        ApiDoc::generate_openapi_json("./".into())?;

        let host = self.host.clone();
        let port = self.port;
        let state = self.state.clone();

        runner.spawn_loop(move |ctx| async move {
            let address = format!("{host}:{port}");
            let socket_addr: SocketAddr = address.parse()?;
            let listener = TcpListener::bind(socket_addr).await?;

            #[allow(clippy::default_constructed_unit_structs)]
            let app = api_router::<ApiDoc>(state.clone())
                .with_state(state)
                // include trace context as header into the response
                //start OpenTelemetry trace on incoming request
                .layer(OtelAxumLayer::default())
                .layer(OtelInResponseLayer::default())
                .layer(CorsLayer::permissive());

            tracing::info!("🧩 Crawler API started at http://{}", socket_addr);

            // Create a shutdown signal from our context
            let token = ctx.token.clone();
            let shutdown = async move { token.cancelled().await };

            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .with_graceful_shutdown(shutdown)
            .await
            .context("😱 API server stopped!")
        });

        Ok(())
    }
}
