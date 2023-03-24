use std::collections::HashMap;

use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attr(Cow<'static, str>);

impl Attr {
    pub const fn new(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

/// `llb.customname`
const CUSTOM_NAME: Attr = Attr::new("llb.customname");

/// `git.keepgitdir`
const KEEP_GIT_DIR: Attr = Attr::new("git.keepgitdir");
/// `git.fullurl`
const FULL_REMOTE_URL: Attr = Attr::new("git.fullurl");
/// `git.authheadersecret`
const AUTH_HEADER_SECRET: Attr = Attr::new("git.authheadersecret");
/// `git.authtokensecret`
const AUTH_TOKEN_SECRET: Attr = Attr::new("git.authtokensecret");
/// `git.knownsshhosts`
const KNOWN_SSH_HOSTS: Attr = Attr::new("git.knownsshhosts");
/// `git.mountsshsock`
const MOUNT_SSH_SOCK: Attr = Attr::new("git.mountsshsock");
/// `local.session`
const LOCAL_SESSION_ID: Attr = Attr::new("local.session");
/// `local.unique`
const LOCAL_UNIQUE_ID: Attr = Attr::new("local.unique");
/// `local.includepattern`
const INCLUDE_PATTERNS: Attr = Attr::new("local.includepattern");
/// `local.followpaths`
const FOLLOW_PATHS: Attr = Attr::new("local.followpaths");
/// `local.excludepatterns`
const EXCLUDE_PATTERNS: Attr = Attr::new("local.excludepatterns");
/// `local.sharedkeyhint`
const SHARED_KEY_HINT: Attr = Attr::new("local.sharedkeyhint");

/// `llbbuild.filename`
const LLB_DEFINITION_FILENAME: Attr = Attr::new("llbbuild.filename");

/// `http.checksum`
const HTTP_CHECKSUM: Attr = Attr::new("http.checksum");
/// `http.filename`
const HTTP_FILENAME: Attr = Attr::new("http.filename");
/// `http.perm`
const HTTP_PERM: Attr = Attr::new("http.perm");
/// `http.uid`
const HTTP_UID: Attr = Attr::new("http.uid");
/// `http.gid`
const HTTP_GID: Attr = Attr::new("http.gid");

/// `image.resolvemode`
const IMAGE_RESOLVE_MODE: Attr = Attr::new("image.resolvemode");
/// `image.recordtype`
const IMAGE_RECORD_TYPE: Attr = Attr::new("image.recordtype");
/// `image.layerlimit`
const IMAGE_LAYER_LIMIT: Attr = Attr::new("image.layerlimit");

pub struct OpMetadata {
    pub ignore_cache: bool,
    pub description: HashMap<Attr, String>,
}

impl OpMetadata {
    pub fn new() -> Self {
        Self {
            ignore_cache: false,
            description: HashMap::default(),
        }
    }
}

pub trait OperationMetadataBuilder {
    fn metadata(&self) -> &OpMetadata;
    fn metadata_mut(&mut self) -> &mut OpMetadata;

    fn ignore_cache(&mut self, ignore: bool) {
        self.metadata_mut().ignore_cache = ignore;
    }

    fn set_description(&mut self, attr: Attr, value: impl AsRef<str>) {
        self.metadata_mut()
            .description
            .insert(attr, value.as_ref().to_owned());
    }

    fn set_custom_name(&mut self, name: impl AsRef<str>) {
        self.set_description(CUSTOM_NAME, name);
    }
}
