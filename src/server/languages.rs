use tonic::{Request, Response, Status};

use crate::pb::{
    languages::languages_server::{Languages as LanguagesService, LanguagesServer},
    languages::{IndexRequest, IndexResponse, Language},
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
        let app = &*self.app;
        let languages = app
            .executors()
            .map(|executor| Language {
                id: executor.id.to_string(),
                name: executor.name.to_string(),
            })
            .collect();
        let response = IndexResponse { languages };
        Ok(Response::new(response))
    }
}
