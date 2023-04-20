pub mod connhelper;
mod error;

use buildkit_rs_llb::Definition;
use buildkit_rs_proto::moby::buildkit::v1::{
    control_client::ControlClient, DiskUsageRequest, DiskUsageResponse, InfoRequest, InfoResponse,
    SolveResponse,
};
use connhelper::docker::docker_connect;
use error::Error;
use tonic::{
    transport::{Channel, Uri},
    IntoRequest, Request, Response,
};
use tower::service_fn;

#[derive(Debug)]
pub struct SolveOptions<'a> {
    /// Should be a random string such as UUID
    pub id: String,
    pub definition: Definition<'a>,
}

#[derive(Debug)]
pub struct Client(ControlClient<Channel>);

impl Client {
    pub async fn connect() -> Result<Client, Error> {
        let channel = Channel::from_static("http://[::1]:50051")
            .connect_with_connector(service_fn(|_: Uri| docker_connect("buildkitd")))
            .await?;

        Ok(Client(ControlClient::new(channel)))
    }

    pub async fn info(&mut self) -> Result<InfoResponse, tonic::Status> {
        self.0.info(InfoRequest {}).await.map(Response::into_inner)
    }

    pub async fn disk_usage(&mut self) -> Result<DiskUsageResponse, tonic::Status> {
        self.0
            .disk_usage(DiskUsageRequest { filter: vec![] })
            .await
            .map(Response::into_inner)
    }

    pub async fn solve(
        &mut self,
        options: SolveOptions<'_>,
    ) -> Result<SolveResponse, tonic::Status> {
        self.0
            .solve(Request::new(
                buildkit_rs_proto::moby::buildkit::v1::SolveRequest {
                    r#ref: options.id,
                    definition: Some(options.definition.into_pb()),
                    frontend_attrs: [("no-cache".to_owned(), "".to_owned())]
                        .into_iter()
                        .collect(),
                    session: todo!(),
                    // exporter: todo!(),
                    // exporter_attrs: todo!(),
                    // frontend: todo!(),
                    // cache: todo!(),
                    // entitlements: todo!(),
                    // frontend_inputs: todo!(),
                    // internal: todo!(),
                    // source_policy: todo!(),
                    ..Default::default()
                },
            ))
            .await
            .map(Response::into_inner)
    }

    pub async fn session(&mut self) -> Result<(), tonic::Status> {
        self.0.session(Request::new(())).await.map(|_| ()
    }
}

trait BuildkitRequestBuilder<T> {
    fn with_buildid(self, buildid: &str) -> Request<T>;
}

impl<T: IntoRequest<Req>, Req> BuildkitRequestBuilder<Req> for T {
    fn with_buildid(self, buildid: &str) -> Request<Req> {
        let mut request = self.into_request();
        request
            .metadata_mut()
            .insert("buildkit-controlapi-buildid", buildid.parse().unwrap());
        request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let mut conn = Client::connect().await.unwrap();
        dbg!(conn.info().await.unwrap());
    }
}
