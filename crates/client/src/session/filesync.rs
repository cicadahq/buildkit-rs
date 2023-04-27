use std::collections::HashMap;
use std::path::{Path, PathBuf};

use buildkit_rs_proto::{
    fsutil::types::{packet::PacketType, Packet, Stat},
    moby::filesync::v1::file_sync_server::{FileSync, FileSyncServer},
};
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use tracing::{debug, error, info, trace, warn};

use crate::util::file_mode::FileMode;

const KEY_INCLUDE_PATTERNS: &str = "include-patterns";
const KEY_EXCLUDE_PATTERNS: &str = "exclude-patterns";
// const KEY_FOLLOW_PATHS: &str = "followpaths";
const KEY_DIR_NAME: &str = "dir-name";

const MAX_PACKET_SIZE: usize = 1024 * 1024 * 4;

pub struct FileSyncService {
    context: HashMap<String, PathBuf>,
}

impl FileSyncService {
    pub fn new(context: HashMap<String, PathBuf>) -> Self {
        Self { context }
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

        info!(?request);

        let dir_name = match request.metadata().get(KEY_DIR_NAME).map(|v| v.to_str()) {
            Some(Ok(dir_name)) => dir_name,
            Some(Err(e)) => {
                return Err(Status::invalid_argument(format!("invalid dir-name: {}", e)))
            }
            None => return Err(Status::invalid_argument("missing dir-name in metadata")),
        };

        let context_path = match self.context.get(dir_name) {
            Some(path) => path.clone(),
            None => return Err(Status::invalid_argument("dir-name not found in context")),
        };

        let include_patterns: Vec<String> = request
            .metadata()
            .get_all(KEY_INCLUDE_PATTERNS)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .map(Into::into)
            .collect();

        let exclude_patterns: Vec<String> = request
            .metadata()
            .get_all(KEY_EXCLUDE_PATTERNS)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .map(Into::into)
            .collect();

        // let follow_paths: Vec<String> = request
        //     .metadata()
        //     .get_all(KEY_FOLLOW_PATHS)
        //     .iter()
        //     .map(|v| v.to_str().ok())
        //     .flatten()
        //     .map(Into::into)
        //     .collect();

        tokio::spawn(async move {
            let files = walk(
                &context_path,
                tx.clone(),
                exclude_patterns,
                include_patterns,
            )
            .await;

            let mut inner = request.into_inner();
            while let Ok(Some(packet)) = inner.message().await {
                trace!(?packet);
                match packet.r#type() {
                    PacketType::PacketReq => {
                        let id = packet.id;
                        let path = context_path.join(&files[id as usize]);

                        debug!(?id, ?path, "Request Packet");

                        let reader = tokio::fs::File::open(&path).await.unwrap();
                        let mut buf_reader = tokio::io::BufReader::new(reader);
                        let mut buffer = vec![0; MAX_PACKET_SIZE];

                        loop {
                            buffer.clear();
                            match buf_reader.read(&mut buffer).await {
                                Ok(0) => {
                                    break;
                                }
                                Ok(n) => {
                                    if let Err(err) = tx
                                        .send(Ok(Packet {
                                            r#type: PacketType::PacketData.into(),
                                            id,
                                            data: buffer[..n].to_vec(),
                                            ..Default::default()
                                        }))
                                        .await
                                    {
                                        error!(?err, "Error sending data packet");
                                    }
                                }
                                Err(err) => {
                                    error!(?err, "Error reading file");
                                    break;
                                }
                            }
                        }

                        // send one with empty data to indicate end of file
                        if let Err(err) = tx
                            .send(Ok(Packet {
                                r#type: PacketType::PacketData.into(),
                                id,
                                ..Default::default()
                            }))
                            .await
                        {
                            error!(?err, "Error sending data packet");
                        }
                    }
                    PacketType::PacketErr => {
                        error!(str =% String::from_utf8_lossy(&packet.data), "Error Packet");
                        break;
                    }
                    PacketType::PacketFin => {
                        info!("fin");
                        if let Err(err) = tx
                            .send(Ok(Packet {
                                r#type: PacketType::PacketFin.into(),
                                ..Default::default()
                            }))
                            .await
                        {
                            error!(?err, "Error sending fin packet");
                        }
                        break;
                    }
                    other => {
                        error!(?other, "Unexpected packet type");
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    #[tracing::instrument(skip_all)]
    async fn tar_stream(
        &self,
        _request: Request<Streaming<Packet>>,
    ) -> Result<Response<Self::TarStreamStream>, Status> {
        warn!("not implemented");
        Err(Status::unimplemented("not implemented"))
    }
}

async fn walk(
    root: impl AsRef<Path>,
    tx: Sender<Result<Packet, Status>>,

    exclude_patterns: Vec<String>,
    _include_patterns: Vec<String>,
) -> Vec<String> {
    macro_rules! send_data_packet {
        ($t:ident, $data:expr) => {
            let _ = tx
                .send(Ok(Packet {
                    r#type: PacketType::$t.into(),
                    data: $data,
                    ..Default::default()
                }))
                .await;
        };
    }

    let root = root.as_ref();
    let mut files = vec![];

    for entry in walkdir::WalkDir::new(root)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            // Very primitive filtering
            // TODO: replace with real matching based on https://github.com/moby/patternmatcher/blob/main/patternmatcher.go
            let trimmed_path = entry.path().strip_prefix(root).unwrap();
            let clean_path = path_clean::clean(trimmed_path);

            !exclude_patterns
                .iter()
                .any(|pattern| clean_path.starts_with(pattern))
        })
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                send_data_packet!(PacketErr, err.to_string().into_bytes());
                continue;
            }
        };

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                send_data_packet!(PacketErr, err.to_string().into_bytes());
                continue;
            }
        };

        let trimmed_path = entry.path().strip_prefix(root).unwrap();
        let clean_path = path_clean::clean(trimmed_path);

        #[cfg(unix)]
        let (uid, gid, size) = {
            use std::os::unix::prelude::MetadataExt;

            let uid = metadata.uid();
            let gid = metadata.gid();
            let size = metadata.size() as i64;

            (uid, gid, size)
        };

        #[cfg(windows)]
        let (uid, gid, size) = {
            use std::os::windows::prelude::MetadataExt;

            // TODO: this seems wrong, not sure what to do here for uid/gid, maybe default to 1000?
            let uid = 0;
            let gid = 0;

            let size = metadata.file_size() as i64;

            (uid, gid, size)
        };

        let stat = Stat {
            path: clean_path.to_string_lossy().into_owned(),
            mode: FileMode::from_metadata(&metadata).bits(),
            uid,
            gid,
            size,
            mod_time: metadata.modified().map_or(0, |t| {
                t.duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64
            }),
            ..Default::default()
        };

        files.push(stat.path.clone());

        if let Err(err) = tx
            .send(Ok(Packet {
                r#type: PacketType::PacketStat.into(),
                stat: Some(stat),
                ..Default::default()
            }))
            .await
        {
            error!(?err);
        }
    }

    // Send a final empty stat packet to indicate the end of the stream.
    if let Err(err) = tx
        .send(Ok(Packet {
            r#type: PacketType::PacketStat.into(),
            ..Default::default()
        }))
        .await
    {
        error!(?err);
    }

    files
}
