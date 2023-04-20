pub mod connhelper;
mod error;

use buildkit_rs_llb::Definition;
use buildkit_rs_proto::moby::buildkit::v1::{
    control_client::ControlClient, DiskUsageRequest, DiskUsageResponse, InfoRequest, InfoResponse,
    ListWorkersRequest, ListWorkersResponse, SolveResponse,
};
use buildkit_rs_util::oci::OciBackend;
use connhelper::{docker::docker_connect, podman::podman_connect};
use error::Error;
use tonic::{
    transport::{Channel, Uri},
    Request, Response,
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
    pub async fn connect(backend: OciBackend, container_name: String) -> Result<Client, Error> {
        let channel = Channel::from_static("http://[::1]:50051")
            .connect_with_connector(service_fn(move |_: Uri| {
                let container_name = container_name.clone();
                async move {
                    match backend {
                        OciBackend::Docker => docker_connect(container_name).await,
                        OciBackend::Podman => podman_connect(container_name).await,
                    }
                }
            }))
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

    pub async fn list_workers(&mut self) -> Result<ListWorkersResponse, tonic::Status> {
        self.0
            .list_workers(ListWorkersRequest { filter: vec![] })
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
                    // session: todo!(),
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

    // pub async fn session(&mut self) -> Result<(), tonic::Status> {
    //     let (incomming_tx, incomming_rx) = tokio::sync::mpsc::channel(1);
    //     let (outgoing_tx, outgoing_rx) = tokio::sync::mpsc::channel(1);

    //     let res = self
    //         .0
    //         .session(tokio_stream::wrappers::ReceiverStream::new(outgoing_rx))
    //         .await?;
    //     let mut inner = res.into_inner();

    //     tokio::spawn(async move {
    //         loop {
    //             match inner.message().await {
    //                 Ok(Some(msg)) => {
    //                     if let Err(_) = incomming_tx.send(msg).await {
    //                         break;
    //                     }
    //                 }
    //                 Ok(None) => {
    //                     break;
    //                 }
    //                 Err(_e) => {
    //                     break;
    //                 }
    //             }
    //         }
    //     });

    //     tokio::spawn(async move {
    //         tonic::transport::Server::builder()
    //             .add_service(todo!())
    //             .serve_with_incoming(tokio_stream::iter(vec![Ok::<_, std::io::Error>(server)]))
    //             .await
    //     });

    //     // tokio::spawn(async move {
    //     //     while let Some(msg) = rx.recv().await {
    //     //         dbg!(msg);
    //     //     }
    //     // });

    //     Ok(())
    // }
}

// trait BuildkitRequestBuilder<T> {
//     fn with_buildid(self, buildid: &str) -> Request<T>;
// }

// impl<T: IntoRequest<Req>, Req> BuildkitRequestBuilder<Req> for T {
//     fn with_buildid(self, buildid: &str) -> Request<Req> {
//         let mut request = self.into_request();
//         request
//             .metadata_mut()
//             .insert("buildkit-controlapi-buildid", buildid.parse().unwrap());
//         request
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let mut conn = Client::connect(OciBackend::Docker, "buildkitd".to_owned())
            .await
            .unwrap();
        dbg!(conn.info().await.unwrap());

        // conn.session().await.unwrap();

        // sleep for 5 sec
        // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
