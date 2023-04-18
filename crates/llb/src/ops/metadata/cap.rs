use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapID(Cow<'static, str>);

impl CapID {
    const fn new(s: &'static str) -> Self {
        CapID(Cow::Borrowed(s))
    }

    pub const SOURCE_IMAGE: CapID = CapID::new("source.image");
    pub const SOURCE_IMAGE_RESOLVE_MODE: CapID = CapID::new("source.image.resolvemode");
    pub const SOURCE_IMAGE_LAYER_LIMIT: CapID = CapID::new("source.image.layerlimit");

    pub const SOURCE_LOCAL: CapID = CapID::new("source.local");
    pub const SOURCE_LOCAL_UNIQUE: CapID = CapID::new("source.local.unique");
    pub const SOURCE_LOCAL_SESSION_ID: CapID = CapID::new("source.local.sessionid");
    pub const SOURCE_LOCAL_INCLUDE_PATTERNS: CapID = CapID::new("source.local.includepatterns");
    pub const SOURCE_LOCAL_FOLLOW_PATHS: CapID = CapID::new("source.local.followpaths");
    pub const SOURCE_LOCAL_EXCLUDE_PATTERNS: CapID = CapID::new("source.local.excludepatterns");
    pub const SOURCE_LOCAL_SHARED_KEY_HINT: CapID = CapID::new("source.local.sharedkeyhint");
    pub const SOURCE_LOCAL_DIFFER: CapID = CapID::new("source.local.differ");

    pub const SOURCE_GIT: CapID = CapID::new("source.git");
    pub const SOURCE_GIT_KEEP_DIR: CapID = CapID::new("source.git.keepgitdir");
    pub const SOURCE_GIT_FULL_URL: CapID = CapID::new("source.git.fullurl");
    pub const SOURCE_GIT_HTTP_AUTH: CapID = CapID::new("source.git.httpauth");
    pub const SOURCE_GIT_KNOWN_SSH_HOSTS: CapID = CapID::new("source.git.knownsshhosts");
    pub const SOURCE_GIT_MOUNT_SSH_SOCK: CapID = CapID::new("source.git.mountsshsock");
    pub const SOURCE_GIT_SUBDIR: CapID = CapID::new("source.git.subdir");

    pub const SOURCE_HTTP: CapID = CapID::new("source.http");
    pub const SOURCE_HTTP_CHECKSUM: CapID = CapID::new("source.http.checksum");
    pub const SOURCE_HTTP_PERM: CapID = CapID::new("source.http.perm");
    pub const SOURCE_HTTP_UID_GID: CapID = CapID::new("soruce.http.uidgid");

    pub const SOURCE_OCI_LAYOUT: CapID = CapID::new("source.ocilayout");

    pub const SOURCE_BUILD_OP_LLB_FILE_NAME: CapID = CapID::new("source.buildop.llbfilename");

    pub const EXEC_META_BASE: CapID = CapID::new("exec.meta.base");
    pub const EXEC_META_CGROUP_PARENT: CapID = CapID::new("exec.meta.cgroup.parent");
    pub const EXEC_META_NETWORK: CapID = CapID::new("exec.meta.network");
    pub const EXEC_META_PROXY: CapID = CapID::new("exec.meta.proxyenv");
    pub const EXEC_META_SECURITY: CapID = CapID::new("exec.meta.security");
    pub const EXEC_META_SECURITY_DEVICE_WHITELIST_V1: CapID =
        CapID::new("exec.meta.security.devices.v1");
    pub const EXEC_META_SETS_DEFAULT_PATH: CapID = CapID::new("exec.meta.setsdefaultpath");
    pub const EXEC_META_ULIMIT: CapID = CapID::new("exec.meta.ulimit");
    pub const EXEC_META_REMOVE_MOUNT_STUBS_RECURSIVE: CapID =
        CapID::new("exec.meta.removemountstubs.recursive");
    pub const EXEC_MOUNT_BIND: CapID = CapID::new("exec.mount.bind");
    pub const EXEC_MOUNT_BIND_READ_WRITE_NO_OUTPUT: CapID =
        CapID::new("exec.mount.bind.readwrite-nooutput");
    pub const EXEC_MOUNT_CACHE: CapID = CapID::new("exec.mount.cache");
    pub const EXEC_MOUNT_CACHE_SHARING: CapID = CapID::new("exec.mount.cache.sharing");
    pub const EXEC_MOUNT_SELECTOR: CapID = CapID::new("exec.mount.selector");
    pub const EXEC_MOUNT_TMPFS: CapID = CapID::new("exec.mount.tmpfs");
    pub const EXEC_MOUNT_TMPFS_SIZE: CapID = CapID::new("exec.mount.tmpfs.size");
    pub const EXEC_MOUNT_SECRET: CapID = CapID::new("exec.mount.secret");
    pub const EXEC_MOUNT_SSH: CapID = CapID::new("exec.mount.ssh");
    pub const EXEC_CGROUPS_MOUNTED: CapID = CapID::new("exec.cgroup");
    pub const EXEC_SECRET_ENV: CapID = CapID::new("exec.secretenv");

    pub const FILE_BASE: CapID = CapID::new("file.base");
    pub const FILE_RM_WILDCARD: CapID = CapID::new("file.rm.wildcard");
    pub const FILE_COPY_INCLUDE_EXCLUDE_PATTERNS: CapID =
        CapID::new("file.copy.includeexcludepatterns");
    pub const FILE_RM_NO_FOLLOW_SYMLINK: CapID = CapID::new("file.rm.nofollowsymlink");

    pub const CONSTRAINTS: CapID = CapID::new("constraints");
    pub const PLATFORM: CapID = CapID::new("platform");

    pub const META_IGNORE_CACHE: CapID = CapID::new("meta.ignorecache");
    pub const META_DESCRIPTION: CapID = CapID::new("meta.description");
    pub const META_EXPORT_CACHE: CapID = CapID::new("meta.exportcache");

    pub const REMOTE_CACHE_GHA: CapID = CapID::new("cache.gha");
    pub const REMOTE_CACHE_S3: CapID = CapID::new("cache.s3");
    pub const REMOTE_CACHE_AZ_BLOB: CapID = CapID::new("cache.azblob");

    pub const MERGE_OP: CapID = CapID::new("mergeop");
    pub const DIFF_OP: CapID = CapID::new("diffop");

    pub const EXPORTER_IMAGE_ANNOTATIONS: CapID = CapID::new("exporter.image.annotations");
    pub const EXPORTER_IMAGE_ATTESTATIONS: CapID = CapID::new("exporter.image.attestations");
    pub const SOURCE_DATE_EPOCH: CapID = CapID::new("exporter.sourcedateepoch");

    pub const SOURCE_POLICY: CapID = CapID::new("source.policy");
}

impl From<CapID> for String {
    fn from(val: CapID) -> Self {
        val.0.into_owned()
    }
}

impl AsRef<str> for CapID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
