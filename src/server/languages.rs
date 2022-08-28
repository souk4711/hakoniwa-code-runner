use tonic::{Request, Response, Status};

use crate::pb::languages::{
    languages_server::{Languages as LanguagesService, LanguagesServer},
    IndexRequest, IndexResponse,
};

pub fn service(server: super::Server) -> LanguagesServer<super::Server> {
    LanguagesServer::new(server)
}

#[tonic::async_trait]
impl LanguagesService for super::Server {
    async fn index(
        &self,
        _request: Request<IndexRequest>,
    ) -> Result<Response<IndexResponse>, Status> {
        let response = IndexResponse { languages: vec![] };
        Ok(Response::new(response))
    }
}
