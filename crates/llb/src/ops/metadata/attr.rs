use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attr(Cow<'static, str>);

impl Attr {
    pub const fn new(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }

    /// `llb.customname`
    pub const CUSTOM_NAME: Attr = Attr::new("llb.customname");

    /// `git.keepgitdir`
    pub const KEEP_GIT_DIR: Attr = Attr::new("git.keepgitdir");
    /// `git.fullurl`
    pub const FULL_REMOTE_URL: Attr = Attr::new("git.fullurl");
    /// `git.authheadersecret`
    pub const AUTH_HEADER_SECRET: Attr = Attr::new("git.authheadersecret");
    /// `git.authtokensecret`
    pub const AUTH_TOKEN_SECRET: Attr = Attr::new("git.authtokensecret");
    /// `git.knownsshhosts`
    pub const KNOWN_SSH_HOSTS: Attr = Attr::new("git.knownsshhosts");
    /// `git.mountsshsock`
    pub const MOUNT_SSH_SOCK: Attr = Attr::new("git.mountsshsock");

    /// `local.session`
    pub const LOCAL_SESSION_ID: Attr = Attr::new("local.session");
    /// `local.unique`
    pub const LOCAL_UNIQUE_ID: Attr = Attr::new("local.unique");
    /// `local.includepattern`
    pub const INCLUDE_PATTERNS: Attr = Attr::new("local.includepattern");
    /// `local.followpaths`
    pub const FOLLOW_PATHS: Attr = Attr::new("local.followpaths");
    /// `local.excludepatterns`
    pub const EXCLUDE_PATTERNS: Attr = Attr::new("local.excludepatterns");
    /// `local.sharedkeyhint`
    pub const SHARED_KEY_HINT: Attr = Attr::new("local.sharedkeyhint");

    /// `llbbuild.filename`
    pub const LLB_DEFINITION_FILENAME: Attr = Attr::new("llbbuild.filename");

    /// `http.checksum`
    pub const HTTP_CHECKSUM: Attr = Attr::new("http.checksum");
    /// `http.filename`
    pub const HTTP_FILENAME: Attr = Attr::new("http.filename");
    /// `http.perm`
    pub const HTTP_PERM: Attr = Attr::new("http.perm");
    /// `http.uid`
    pub const HTTP_UID: Attr = Attr::new("http.uid");
    /// `http.gid`
    pub const HTTP_GID: Attr = Attr::new("http.gid");

    /// `image.resolvemode`
    pub const IMAGE_RESOLVE_MODE: Attr = Attr::new("image.resolvemode");
    /// `image.recordtype`
    pub const IMAGE_RECORD_TYPE: Attr = Attr::new("image.recordtype");
    /// `image.layerlimit`
    pub const IMAGE_LAYER_LIMIT: Attr = Attr::new("image.layerlimit");
}

impl From<Attr> for String {
    fn from(val: Attr) -> Self {
        val.0.into_owned()
    }
}

impl AsRef<str> for Attr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
