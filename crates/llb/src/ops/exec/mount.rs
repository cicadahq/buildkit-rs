use buildkit_rs_proto::pb::{
    CacheOpt, CacheSharingOpt, Mount as PbMount, MountType as PbMountType, SecretOpt, SshOpt,
    TmpfsOpt,
};
use camino::Utf8PathBuf;

use crate::utils::{OperationOutput, OutputIdx};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CacheSharingMode {
    #[default]
    Shared,
    Private,
    Locked,
}

impl From<CacheSharingMode> for CacheSharingOpt {
    fn from(mode: CacheSharingMode) -> Self {
        match mode {
            CacheSharingMode::Shared => Self::Shared,
            CacheSharingMode::Private => Self::Private,
            CacheSharingMode::Locked => Self::Locked,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MountType<'a> {
    Scratch {
        output: OutputIdx,
    },
    Layer {
        input: OperationOutput<'a>,
        /// Readonly if None
        output: Option<OutputIdx>,
    },
    Tmpfs {
        size: i64,
    },
    Cache {
        id: String,
        sharing: CacheSharingMode,
    },
    Secret {
        id: String,
        uid: u32,
        gid: u32,
        mode: u32,
        optional: bool,
    },
    Ssh {
        id: String,
        uid: u32,
        gid: u32,
        mode: u32,
        optional: bool,
    },
}

#[derive(Debug, Clone)]
pub struct Mount<'a> {
    dest: Utf8PathBuf,
    mount_type: MountType<'a>,
    /// Selector for layer
    selector: Option<String>,
}

impl Mount<'_> {
    pub fn scratch(dest: impl Into<Utf8PathBuf>, output: impl Into<OutputIdx>) -> Mount<'static> {
        Mount {
            dest: dest.into(),
            mount_type: MountType::Scratch {
                output: output.into(),
            },
            selector: None,
        }
    }

    pub fn layer(
        input: OperationOutput<'_>,
        dest: impl Into<Utf8PathBuf>,
        output: impl Into<OutputIdx>,
    ) -> Mount<'_> {
        Mount {
            dest: dest.into(),
            mount_type: MountType::Layer {
                input,
                output: Some(output.into()),
            },
            selector: None,
        }
    }

    pub fn layer_readonly(input: OperationOutput<'_>, dest: impl Into<Utf8PathBuf>) -> Mount<'_> {
        Mount {
            dest: dest.into(),
            mount_type: MountType::Layer {
                input,
                output: None,
            },
            selector: None,
        }
    }

    pub fn cache(
        dest: impl Into<Utf8PathBuf>,
        id: impl Into<String>,
        sharing: CacheSharingMode,
    ) -> Mount<'static> {
        Mount {
            dest: dest.into(),
            mount_type: MountType::Cache {
                id: id.into(),
                sharing,
            },
            selector: None,
        }
    }

    pub fn secret(
        dest: impl Into<Utf8PathBuf>,
        id: impl Into<String>,
        uid: u32,
        gid: u32,
        mode: u32,
        optional: bool,
    ) -> Mount<'static> {
        Mount {
            dest: dest.into(),
            mount_type: MountType::Secret {
                id: id.into(),
                uid,
                gid,
                mode,
                optional,
            },
            selector: None,
        }
    }

    pub fn with_selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }

    pub(crate) fn input(&self) -> Option<&OperationOutput> {
        match &self.mount_type {
            MountType::Layer { input, .. } => Some(input),
            _ => None,
        }
    }

    pub(crate) fn to_pb(&self, input: i64) -> PbMount {
        PbMount {
            input,

            output: match &self.mount_type {
                MountType::Scratch { output } => Some(output.into()),
                MountType::Layer { output, .. } => output.map(|o| o.into()),
                _ => None,
            }
            .unwrap_or(-1),

            selector: self.selector.clone().unwrap_or_default(),
            dest: self.dest.clone().into(),

            // TODO: support result_id
            result_id: "".into(),

            readonly: match &self.mount_type {
                MountType::Layer { output, .. } => output.is_none(),
                _ => false,
            },

            mount_type: match self.mount_type {
                MountType::Layer { .. } | MountType::Scratch { .. } => PbMountType::Bind,
                MountType::Tmpfs { .. } => PbMountType::Tmpfs,
                MountType::Cache { .. } => PbMountType::Cache,
                MountType::Secret { .. } => PbMountType::Secret,
                MountType::Ssh { .. } => PbMountType::Ssh,
            }
            .into(),

            tmpfs_opt: match &self.mount_type {
                MountType::Tmpfs { size } => Some(TmpfsOpt { size: *size }),
                _ => None,
            },

            cache_opt: match &self.mount_type {
                MountType::Cache { id, sharing } => {
                    let sharing_pb: CacheSharingOpt = (*sharing).into();
                    Some(CacheOpt {
                        id: id.clone(),
                        sharing: sharing_pb.into(),
                    })
                }
                _ => None,
            },

            secret_opt: match &self.mount_type {
                MountType::Secret {
                    id,
                    uid,
                    gid,
                    mode,
                    optional,
                } => Some(SecretOpt {
                    id: id.clone(),
                    uid: *uid,
                    gid: *gid,
                    mode: *mode,
                    optional: *optional,
                }),
                _ => None,
            },

            ssh_opt: match &self.mount_type {
                MountType::Ssh {
                    id,
                    uid,
                    gid,
                    mode,
                    optional,
                } => Some(SshOpt {
                    id: id.clone(),
                    uid: *uid,
                    gid: *gid,
                    mode: *mode,
                    optional: *optional,
                }),
                _ => None,
            },
        }
    }
}
