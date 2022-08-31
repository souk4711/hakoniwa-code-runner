use chrono::prelude::*;
use tonic::{Request, Response, Status};

use crate::{
    pb::{
        runs::runs_server::{Runs as RunsService, RunsServer},
        runs::{CreateRequest, CreateResponse, Duration},
    },
    ExecutorFile,
};

pub fn service(server: super::Server) -> RunsServer<super::Server> {
    RunsServer::new(server)
}

#[tonic::async_trait]
impl RunsService for super::Server {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let request = request.into_inner();
        let app = &*self.app;

        let executor = app
            .get_executor(&request.lang)
            .ok_or_else(|| Status::invalid_argument("lang is invalid"))?;
        let r = executor.run(
            &request
                .files
                .into_iter()
                .map(|f| ExecutorFile::new(&f.name, &f.content))
                .collect::<Vec<_>>(),
        );

        let response = CreateResponse {
            status: r.status,
            reason: r.reason,
            stdout: r.stdout,
            stderr: r.stderr,
            exit_code: r.exit_code,
            start_time: r
                .start_time
                .map(|v| v.to_rfc3339_opts(SecondsFormat::Nanos, true)),
            real_time: r.real_time.map(Duration::from),
            system_time: r.system_time.map(Duration::from),
            user_time: r.user_time.map(Duration::from),
            max_rss: r.max_rss,
        };
        Ok(Response::new(response))
    }
}
