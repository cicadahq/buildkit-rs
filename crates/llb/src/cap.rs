use std::borrow::Cow;

struct CapID(Cow<'static, str>);

impl CapID {
    const fn new(s: &'static str) -> Self {
        CapID(Cow::Borrowed(s))
    }
}

const SOURCE_IMAGE: CapID = CapID::new("source.image");
const SOURCE_IMAGE_RESOLVE_MODE: CapID = CapID::new("source.image.resolvemode");
const SOURCE_IMAGE_LAYER_LIMIT: CapID = CapID::new("source.image.layerlimit");

const SOURCE_LOCAL: CapID = CapID::new("source.local");
const SOURCE_LOCAL_UNIQUE: CapID = CapID::new("source.local.unique");
const SOURCE_LOCAL_SESSION_ID: CapID = CapID::new("source.local.sessionid");
const SOURCE_LOCAL_INCLUDE_PATTERNS: CapID = CapID::new("source.local.includepatterns");
const SOURCE_LOCAL_FOLLOW_PATHS: CapID = CapID::new("source.local.followpaths");
const SOURCE_LOCAL_EXCLUDE_PATTERNS: CapID = CapID::new("source.local.excludepatterns");
const SOURCE_LOCAL_SHARED_KEY_HINT: CapID = CapID::new("source.local.sharedkeyhint");
const SOURCE_LOCAL_DIFFER: CapID = CapID::new("source.local.differ");

const SOURCE_GIT: CapID = CapID::new("source.git");
const SOURCE_GIT_KEEP_DIR: CapID = CapID::new("source.git.keepgitdir");
const SOURCE_GIT_FULL_URL: CapID = CapID::new("source.git.fullurl");
const SOURCE_GIT_HTTP_AUTH: CapID = CapID::new("source.git.httpauth");
const SOURCE_GIT_KNOWN_SSH_HOSTS: CapID = CapID::new("source.git.knownsshhosts");
const SOURCE_GIT_MOUNT_SSH_SOCK: CapID = CapID::new("source.git.mountsshsock");
const SOURCE_GIT_SUBDIR: CapID = CapID::new("source.git.subdir");

const SOURCE_HTTP: CapID = CapID::new("source.http");
const SOURCE_HTTP_CHECKSUM: CapID = CapID::new("source.http.checksum");
const SOURCE_HTTP_PERM: CapID = CapID::new("source.http.perm");
const SOURCE_HTTP_UID_GID: CapID = CapID::new("soruce.http.uidgid");

const SOURCE_OCI_LAYOUT: CapID = CapID::new("source.ocilayout");

const SOURCE_BUILD_OP_LLB_FILE_NAME: CapID = CapID::new("source.buildop.llbfilename");

const EXEC_META_BASE: CapID = CapID::new("exec.meta.base");
const EXEC_META_CGROUP_PARENT: CapID = CapID::new("exec.meta.cgroup.parent");
const EXEC_META_NETWORK: CapID = CapID::new("exec.meta.network");
const EXEC_META_PROXY: CapID = CapID::new("exec.meta.proxyenv");
const EXEC_META_SECURITY: CapID = CapID::new("exec.meta.security");
const EXEC_META_SECURITY_DEVICE_WHITELIST_V1: CapID = CapID::new("exec.meta.security.devices.v1");
const EXEC_META_SETS_DEFAULT_PATH: CapID = CapID::new("exec.meta.setsdefaultpath");
const EXEC_META_ULIMIT: CapID = CapID::new("exec.meta.ulimit");
const EXEC_META_REMOVE_MOUNT_STUBS_RECURSIVE: CapID =
    CapID::new("exec.meta.removemountstubs.recursive");
const EXEC_MOUNT_BIND: CapID = CapID::new("exec.mount.bind");
const EXEC_MOUNT_BIND_READ_WRITE_NO_OUTPUT: CapID =
    CapID::new("exec.mount.bind.readwrite-nooutput");
const EXEC_MOUNT_CACHE: CapID = CapID::new("exec.mount.cache");
const EXEC_MOUNT_CACHE_SHARING: CapID = CapID::new("exec.mount.cache.sharing");
const EXEC_MOUNT_SELECTOR: CapID = CapID::new("exec.mount.selector");
const EXEC_MOUNT_TMPFS: CapID = CapID::new("exec.mount.tmpfs");
const EXEC_MOUNT_TMPFS_SIZE: CapID = CapID::new("exec.mount.tmpfs.size");
const EXEC_MOUNT_SECRET: CapID = CapID::new("exec.mount.secret");
const EXEC_MOUNT_SSH: CapID = CapID::new("exec.mount.ssh");
const EXEC_CGROUPS_MOUNTED: CapID = CapID::new("exec.cgroup");
const EXEC_SECRET_ENV: CapID = CapID::new("exec.secretenv");

const FILE_BASE: CapID = CapID::new("file.base");
const FILE_RM_WILDCARD: CapID = CapID::new("file.rm.wildcard");
const FILE_COPY_INCLUDE_EXCLUDE_PATTERNS: CapID = CapID::new("file.copy.includeexcludepatterns");
const FILE_RM_NO_FOLLOW_SYMLINK: CapID = CapID::new("file.rm.nofollowsymlink");

const CONSTRAINTS: CapID = CapID::new("constraints");
const PLATFORM: CapID = CapID::new("platform");

const META_IGNORE_CACHE: CapID = CapID::new("meta.ignorecache");
const META_DESCRIPTION: CapID = CapID::new("meta.description");
const META_EXPORT_CACHE: CapID = CapID::new("meta.exportcache");

const REMOTE_CACHE_GHA: CapID = CapID::new("cache.gha");
const REMOTE_CACHE_S3: CapID = CapID::new("cache.s3");
const REMOTE_CACHE_AZ_BLOB: CapID = CapID::new("cache.azblob");

const MERGE_OP: CapID = CapID::new("mergeop");
const DIFF_OP: CapID = CapID::new("diffop");

const EXPORTER_IMAGE_ANNOTATIONS: CapID = CapID::new("exporter.image.annotations");
const EXPORTER_IMAGE_ATTESTATIONS: CapID = CapID::new("exporter.image.attestations");
const SOURCE_DATE_EPOCH: CapID = CapID::new("exporter.sourcedateepoch");

const SOURCE_POLICY: CapID = CapID::new("source.policy");
