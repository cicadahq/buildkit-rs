pub mod connhelper;
pub(crate) mod error;
pub mod session;
pub(crate) mod util;

use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

use buildkit_rs_llb::Definition;
use buildkit_rs_proto::moby::buildkit::secrets::v1::secrets_server::SecretsServer;
use buildkit_rs_proto::moby::buildkit::v1::BytesMessage;
use buildkit_rs_proto::moby::buildkit::v1::{
    control_client::ControlClient, DiskUsageRequest, DiskUsageResponse, InfoRequest, InfoResponse,
    ListWorkersRequest, ListWorkersResponse, SolveResponse,
};
use buildkit_rs_proto::moby::buildkit::v1::{StatusRequest, StatusResponse};
use buildkit_rs_proto::moby::filesync::v1::auth_server::AuthServer;
use buildkit_rs_proto::moby::filesync::v1::file_sync_server::FileSyncServer;
use buildkit_rs_util::oci::OciBackend;
use connhelper::{docker::docker_connect, podman::podman_connect};
use error::Error;
use futures::stream::StreamExt;
use session::filesend::FileSendService;
use session::secret::SecretSource;
use session::{auth::AuthService, filesync::FileSyncService};
use tokio::io::AsyncWriteExt;
use tokio_util::io::ReaderStream;
use tonic::{
    transport::{Channel, Uri},
    Request, Response,
};
use tonic::{Status, Streaming};
use tower::{service_fn, ServiceBuilder};
use tower_http::ServiceBuilderExt;
use tracing::{debug, info};

use crate::session::secret::SecretService;
pub use crate::util::id::random_id;

const HEADER_SESSION_ID: &str = "x-docker-expose-session-uuid";
const HEADER_SESSION_NAME: &str = "x-docker-expose-session-name";
const HEADER_SESSION_SHARED_KEY: &str = "x-docker-expose-session-sharedkey";
const HEADER_SESSION_METHOD: &str = "x-docker-expose-session-grpc-method";

#[derive(Debug)]
pub struct SolveOptions<'a> {
    pub id: String,
    pub session: String,
    pub definition: Definition<'a>,
}

#[derive(Debug, Clone, Default)]
pub struct SessionOptions {
    pub name: String,
    pub local: HashMap<String, PathBuf>,
    pub secrets: HashMap<String, SecretSource>,
}

pub struct Session {
    pub id: String,
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
        let config = oci_spec::image::ConfigBuilder::default()
            .user("root".to_string())
            // .working_dir(job.working_directory.clone())
            .env(["ABC=123".to_owned()])
            .cmd(["/bin/bash".to_owned()])
            // .entrypoint(["/app/hello-world".to_owned()])
            .build()
            .unwrap();

        let image_config = oci_spec::image::ImageConfigurationBuilder::default()
            .config(config)
            .build()
            .unwrap();

        let json = serde_json::to_string(&image_config).unwrap();

        self.0
            .solve(Request::new(
                buildkit_rs_proto::moby::buildkit::v1::SolveRequest {
                    r#ref: options.id,
                    definition: Some(options.definition.into_pb()),
                    frontend_attrs: [("no-cache".to_owned(), "".to_owned())]
                        .into_iter()
                        .collect(),
                    session: options.session,
                    exporter: "docker".to_owned(),
                    exporter_attrs: [
                        ("name".into(), "test".into()),
                        ("containerimage.config".into(), json),
                    ]
                    .into_iter()
                    .collect(),
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
            .map(|res| res.into_inner())
    }

    pub async fn session(&mut self, options: SessionOptions) -> Result<Session, tonic::Status> {
        let (server_stream, client_stream) = tokio::io::duplex(4096);

        let (mut health_reporter, health_server) = tonic_health::server::health_reporter();

        let auth = AuthService::new().into_server();
        let file_sync = FileSyncService::new(options.local).into_server();
        let file_send = FileSendService::new().into_server();
        let secret = SecretService::new(options.secrets).into_server();

        health_reporter
            .set_serving::<AuthServer<AuthService>>()
            .await;

        health_reporter
            .set_serving::<FileSyncServer<FileSyncService>>()
            .await;

        health_reporter
            .set_serving::<SecretsServer<SecretService>>()
            .await;

        health_reporter
            .set_serving::<SecretsServer<SecretService>>()
            .await;

        let layer = ServiceBuilder::new().trace_for_grpc().into_inner();

        tokio::spawn(async move {
            match tonic::transport::Server::builder()
                .trace_fn(|_| tracing::info_span!("session server"))
                .layer(layer)
                .add_service(health_server)
                .add_service(auth)
                .add_service(file_sync)
                .add_service(file_send)
                .add_service(secret)
                .serve_with_incoming(futures::stream::iter(vec![Ok::<_, std::io::Error>(
                    server_stream,
                )]))
                .await
            {
                Ok(()) => debug!("Server finished"),
                Err(err) => tracing::error!(?err, "Server error"),
            }
        });

        // In memory client
        // let mut client = Some(client_stream);
        // let channel = Endpoint::try_from("http://[::]:50051")
        //     .unwrap()
        //     .connect_with_connector(service_fn(move |_: Uri| {
        //         let client = client.take();

        //         async move {
        //             if let Some(client) = client {
        //                 Ok(client)
        //             } else {
        //                 Err(std::io::Error::new(
        //                     std::io::ErrorKind::Other,
        //                     "Client already taken",
        //                 ))
        //             }
        //         }
        //     }))
        //     .await
        //     .unwrap();

        // loop {
        //     tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        //     // health check
        //     let mut client = tonic_health::pb::health_client::HealthClient::new(channel.clone());
        //     let res = client
        //         .check(tonic_health::pb::HealthCheckRequest {
        //             // service: "moby.filesync.v1.Auth".into(),
        //             service: "".into()
        //         })
        //         .await;

        //     info!(?res, "Health check");
        // }

        let (client_read, mut client_write) = tokio::io::split(client_stream);

        let mut request = Request::new(ReaderStream::new(client_read).map(|bytes| BytesMessage {
            data: bytes.unwrap().to_vec(),
        }));

        let id = random_id();

        request
            .metadata_mut()
            .append(HEADER_SESSION_ID, id.parse().expect("valid header value"));

        // Map the name to a valid header value so we make sure it doenst panic
        let header_name_bytes = options
            .name
            .bytes()
            .map(|b| if (32..127).contains(&b) { b } else { b'?' })
            .collect::<Vec<_>>();
        let header_name = String::from_utf8_lossy(&header_name_bytes);

        request.metadata_mut().append(
            HEADER_SESSION_NAME,
            header_name.parse().expect("valid header value"),
        );

        request.metadata_mut().append(
            HEADER_SESSION_SHARED_KEY,
            "".parse().expect("valid header value"),
        );

        // request.metadata_mut().append(
        //     HEADER_SESSION_METHOD,
        //     "/moby.filesync.v1.Auth/Credentials"
        //         .parse()
        //         .expect("valid header value"),
        // );

        // request.metadata_mut().append(
        //     HEADER_SESSION_METHOD,
        //     "/moby.filesync.v1.Auth/FetchToken"
        //         .parse()
        //         .expect("valid header value"),
        // );

        // request.metadata_mut().append(
        //     HEADER_SESSION_METHOD,
        //     "/moby.filesync.v1.Auth/GetTokenAuthority"
        //         .parse()
        //         .expect("valid header value"),
        // );

        // request.metadata_mut().append(
        //     HEADER_SESSION_METHOD,
        //     "/moby.filesync.v1.Auth/VerifyTokenAuthority"
        //         .parse()
        //         .expect("valid header value"),
        // );

        // TODO: make these dynamic
        request.metadata_mut().append(
            HEADER_SESSION_METHOD,
            "/moby.filesync.v1.FileSync/DiffCopy"
                .parse()
                .expect("valid header value"),
        );

        request.metadata_mut().append(
            HEADER_SESSION_METHOD,
            "/moby.filesync.v1.FileSync/TarStream"
                .parse()
                .expect("valid header value"),
        );

        request.metadata_mut().append(
            HEADER_SESSION_METHOD,
            "/moby.filesync.v1.FileSend/DiffCopy"
                .parse()
                .expect("valid header value"),
        );

        request.metadata_mut().append(
            HEADER_SESSION_METHOD,
            "/moby.buildkit.secrets.v1.Secrets/GetSecret"
                .parse()
                .expect("valid header value"),
        );

        let res = self.0.session(request).await?;

        tokio::spawn(async move {
            let mut inner = res.into_inner();

            loop {
                match inner.message().await {
                    Ok(Some(msg)) => {
                        if let Err(err) = client_write.write_all(&msg.data).await {
                            tracing::error!(?err, "Error writing to client");
                            break;
                        }
                    }
                    Ok(None) => {
                        info!("Session finished");
                        break;
                    }
                    Err(err) => {
                        tracing::error!(?err, "Error");
                        break;
                    }
                }
            }

            info!("Client finished")
        });

        Ok(Session { id })
    }

    pub async fn status(&mut self, id: String) -> Result<Streaming<StatusResponse>, Status> {
        self.0
            .status(StatusRequest { r#ref: id })
            .await
            .map(Response::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        std::env::set_var("RUST_LOG", "debug");
        tracing_subscriber::fmt::init();

        let mut conn = Client::connect(OciBackend::Docker, "cicada-buildkitd".to_owned())
            .await
            .unwrap();
        dbg!(conn.info().await.unwrap());

        let session = conn
            .session(SessionOptions {
                name: "cicada".into(),
                ..Default::default()
            })
            .await
            .unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(8)).await;

        // sleep for 5 sec
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
