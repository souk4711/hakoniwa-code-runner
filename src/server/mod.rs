mod languages;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Clone)]
pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new() -> Self {
        Self {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
        }
    }

    #[tokio::main]
    pub async fn start(&self) -> anyhow::Result<()> {
        tonic::transport::Server::builder()
            .add_service(languages::service(self.clone()))
            .serve(self.addr)
            .await?;
        Ok(())
    }
}
