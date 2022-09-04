mod languages;
mod runs;

use anyhow::anyhow;
use std::sync::Arc;
use tokio::{signal, sync::oneshot};

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
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET)
            .build()?;

        let config = &self.app.config.server;
        let addr = format!("{}:{}", config.ip, config.port)
            .parse()
            .map_err(|err| anyhow!("Error parsing `server`: {}", err))?;

        let (tx, rx) = oneshot::channel::<()>();
        _ = tokio::spawn(async move {
            _ = signal::ctrl_c().await;
            _ = tx.send(());
        });

        tonic::transport::Server::builder()
            .add_service(reflection_service)
            .add_service(languages::service(self.clone()))
            .add_service(runs::service(self.clone()))
            .serve_with_shutdown(addr, async { _ = rx.await })
            .await?;

        Ok(())
    }
}
