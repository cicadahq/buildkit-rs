use crate::marshal::MarshalCache;
use crate::state::Constraints;
use crate::state::Output;
use crate::state::State;

pub struct Mount {
    pub target: String,
    pub read_only: bool,
    pub source: Box<dyn Output>,
    pub output: Box<dyn Output>,
    pub selector: String,
    pub cache_id: String,
    pub tmpfs: bool,
    pub tmpfs_opt: TmpfsInfo,
    pub cache_sharing: CacheMountSharingMode,
    pub no_output: bool,
}

pub struct ExecOp {
    pub marshal_cache: MarshalCache,
    pub proxy_env: ProxyEnv,
    pub root: Box<dyn Output>,
    pub mounts: Vec<Mount>,
    pub base: State,
    pub constraints: Constraints,
    pub is_validated: bool,
    pub secrets: Vec<SecretInfo>,
    pub ssh: Vec<SSHInfo>,
}

pub struct ExecState {
    state: State,
    exec: Box<ExecOp>,
}

pub type MountOption = Box<dyn Fn(&mut Mount)>;

pub trait TmpfsOption {
    fn set_tmpfs_option(&self, ti: &mut TmpfsInfo);
}

pub struct TmpfsSize {
    b: i64,
}

pub struct TmpfsInfo {
    size: i64,
}

pub trait RunOption {
    fn set_run_option(&self, es: &mut ExecInfo);
}

struct SSHInfo {
    id: String,
    target: String,
    mode: i32,
    uid: i32,
    gid: i32,
    optional: bool,
}

struct SecretInfo {
    id: String,
    target: String,
    mode: i32,
    uid: i32,
    gid: i32,
    optional: bool,
    is_env: bool,
}

struct ExecInfo {
    constraints: Constraints,
    state: State,
    mounts: Vec<MountInfo>,
    readonly_root_fs: bool,
    proxy_env: Option<ProxyEnv>,
    secrets: Vec<SecretInfo>,
    ssh: Vec<SSHInfo>,
}

struct MountInfo {
    target: String,
    source: Box<dyn Output>,
    opts: Vec<MountOption>,
}

struct ProxyEnv {
    http_proxy: Option<String>,
    https_proxy: Option<String>,
    ftp_proxy: Option<String>,
    no_proxy: Option<String>,
    all_proxy: Option<String>,
}

type CacheMountSharingMode = i32;
