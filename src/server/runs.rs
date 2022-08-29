use tonic::{Request, Response, Status};

use crate::pb::{
    runs::runs_server::{Runs as RunsService, RunsServer},
    runs::{CreateRequest, CreateResponse},
};

pub fn service(server: super::Server) -> RunsServer<super::Server> {
    RunsServer::new(server)
}

#[tonic::async_trait]
impl RunsService for super::Server {
    async fn create(
        &self,
        _request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let response = CreateResponse {};
        Ok(Response::new(response))
    }
}
