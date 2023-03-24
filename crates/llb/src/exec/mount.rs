use std::path::Path;

struct TmpfsInfo {
    size: i64,
}

enum CacheMountSharingMode {
    CacheMountShared = 0,
    CacheMountPrivate,
    CacheMountLocked,
}

struct Mount<P: AsRef<Path>> {
    target: P,
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
