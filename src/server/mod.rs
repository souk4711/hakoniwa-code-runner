mod languages;
mod runs;

use anyhow::anyhow;
use http::Request;
use hyper::Body;
use std::sync::Arc;
use tokio::{signal, sync::oneshot};
use tower_http::{
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{Level, Span};
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    prelude::*,
};

use crate::{pb, App};

#[derive(Clone)]
pub struct Server {
    app: Arc<App>,
}

impl Server {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }

    #[tokio::main]
    pub async fn start(&self) -> anyhow::Result<()> {
        // Configuration.
        let config = &self.app.config.server;

        // gRPC Reflection service.
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET)
            .build()?;

        // Tracing feature.
        let fmt_layer = tracing_subscriber::fmt::layer();
        let mut env_filter_layer = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy();
        if let Some(LevelFilter::INFO) = env_filter_layer.max_level_hint() {
            env_filter_layer = env_filter_layer.add_directive("hakoniwa=off".parse()?);
        }
        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(env_filter_layer)
            .init();

        // Graceful shutdown feature.
        let (tx, rx) = oneshot::channel::<()>();
        _ = tokio::spawn(async move {
            _ = signal::ctrl_c().await;
            _ = tx.send(());
        });

        // Start.
        let addr = format!("{}:{}", config.ip, config.port)
            .parse()
            .map_err(|err| anyhow!("Error parsing `server`: {}", err))?;
        tracing::info!(target: "hcr::server", "listening on {}", addr);
        tonic::transport::Server::builder()
            .layer(
                TraceLayer::new_for_grpc()
                    .on_request(|request: &Request<Body>, _span: &Span| {
                        tracing::info!(target: "tower_http::trace::on_request", "started {} {}", request.method(), request.uri().path())
                    })
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            )
            .add_service(reflection_service)
            .add_service(languages::service(self.clone()))
            .add_service(runs::service(self.clone()))
            .serve_with_shutdown(addr, async { _ = rx.await })
            .await?;

        Ok(())
    }
}
