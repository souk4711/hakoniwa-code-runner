mod languages;
mod runs;

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::{signal, sync::oneshot};

use crate::App;

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
        let (tx, rx) = oneshot::channel::<()>();
        _ = tokio::spawn(async move {
            _ = signal::ctrl_c().await;
            _ = tx.send(());
        });

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
        tonic::transport::Server::builder()
            .add_service(languages::service(self.clone()))
            .add_service(runs::service(self.clone()))
            .serve_with_shutdown(addr, async { _ = rx.await })
            .await?;
        Ok(())
    }
}
