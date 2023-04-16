use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
struct TmpfsInfo {
    size: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CacheMountSharingMode {
    CacheMountShared = 0,
    CacheMountPrivate,
    CacheMountLocked,
}

#[derive(Debug, Clone)]
pub struct Mount {
    target: PathBuf,
    readonly: bool,
    // source: Output,
    // output: Output,
    selector: String,
    cache_id: String,
    tmpfs: bool,
    tmpfs_opt: TmpfsInfo,
    cache_sharing: CacheMountSharingMode,
    no_output: bool,
}
