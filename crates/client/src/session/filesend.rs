use buildkit_rs_proto::moby::filesync::v1::{
    file_send_server::{FileSend, FileSendServer},
    BytesMessage,
};
use tokio::io::AsyncWriteExt;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, Status};
use tracing::info;

pub(crate) struct FileSendService {}

impl FileSendService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn into_server(self) -> FileSendServer<Self> {
        FileSendServer::new(self)
    }
}

#[tonic::async_trait]
impl FileSend for FileSendService {
    type DiffCopyStream = ReceiverStream<Result<BytesMessage, Status>>;

    async fn diff_copy(
        &self,
        request: tonic::Request<tonic::Streaming<BytesMessage>>,
    ) -> Result<Response<Self::DiffCopyStream>, Status> {
        info!(?request);

        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let stream = request.into_inner();

        tokio::spawn(async move {
            let mut docker_load = tokio::process::Command::new("docker")
                .arg("load")
                .stdin(std::process::Stdio::piped())
                .spawn()
                .unwrap();

            let mut docker_load_stdin = docker_load.stdin.take().unwrap();
            
            let _tx = tx;
            let mut stream = stream;
            while let Some(message) = stream.message().await.unwrap() {
                let data = message.data;
                docker_load_stdin.write_all(&data).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
