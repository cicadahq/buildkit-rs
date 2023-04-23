use buildkit_rs_proto::{
    fsutil::types::Packet,
    moby::filesync::v1::file_sync_server::{FileSync, FileSyncServer},
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::info;

pub struct FileSyncService;

impl FileSyncService {
    pub fn new() -> Self {
        Self
    }

    pub fn into_server(self) -> FileSyncServer<Self> {
        FileSyncServer::new(self)
    }
}

#[tonic::async_trait]
impl FileSync for FileSyncService {
    type DiffCopyStream = ReceiverStream<Result<Packet, Status>>;
    type TarStreamStream = ReceiverStream<Result<Packet, Status>>;

    #[tracing::instrument(skip_all)]
    async fn diff_copy(
        &self,
        request: Request<Streaming<Packet>>,
    ) -> Result<Response<Self::DiffCopyStream>, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        tokio::spawn(async move {
            let mut inner = request.into_inner();
            while let Ok(Some(packet)) = inner.message().await {
                info!(?packet);
                tx.send(Ok(packet)).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    #[tracing::instrument(skip_all)]
    async fn tar_stream(
        &self,
        request: Request<Streaming<Packet>>,
    ) -> Result<Response<Self::TarStreamStream>, Status> {
        unimplemented!()
    }
}
