/*

type State struct {
    out   Output
    prev  *State
    key   interface{}
    value func(context.Context, *Constraints) (interface{}, error)
    opts  []ConstraintsOpt
    async *asyncState
}
 */

/*
keys

	keyArgs         = contextKeyT("llb.exec.args")
	keyDir          = contextKeyT("llb.exec.dir")
	keyEnv          = contextKeyT("llb.exec.env")
	keyExtraHost    = contextKeyT("llb.exec.extrahost")
	keyHostname     = contextKeyT("llb.exec.hostname")
	keyUlimit       = contextKeyT("llb.exec.ulimit")
	keyCgroupParent = contextKeyT("llb.exec.cgroup.parent")
	keyUser         = contextKeyT("llb.exec.user")

	keyPlatform = contextKeyT("llb.platform")
	keyNetwork  = contextKeyT("llb.network")
	keySecurity = contextKeyT("llb.security")
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ContextKey {
    /// `llb.exec.args`
    Args,
    /// `llb.exec.dir`
    Dir,
    /// `llb.exec.env`
    Env,
    /// `llb.exec.extrahost`
    ExtraHost,
    /// `llb.exec.hostname`
    Hostname,
    /// `llb.exec.ulimit`
    Ulimit,
    /// `llb.exec.cgroup.parent`
    CgroupParent,
    /// `llb.exec.user`
    User,

    /// `llb.platform`
    Platform,
    /// `llb.network`
    Network,
    /// `llb.security`
    Security,
}

impl ContextKey {
    pub fn as_str(&self) -> &str {
        match self {
            ContextKey::Args => "llb.exec.args",
            ContextKey::Dir => "llb.exec.dir",
            ContextKey::Env => "llb.exec.env",
            ContextKey::ExtraHost => "llb.exec.extrahost",
            ContextKey::Hostname => "llb.exec.hostname",
            ContextKey::Ulimit => "llb.exec.ulimit",
            ContextKey::CgroupParent => "llb.exec.cgroup.parent",
            ContextKey::User => "llb.exec.user",
            ContextKey::Platform => "llb.platform",
            ContextKey::Network => "llb.network",
            ContextKey::Security => "llb.security",
        }
    }
}

impl std::fmt::Display for ContextKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


pub fn configure_dir(state: State, dir: String) -> State {
    state
}

pub struct State {
    // out: Output,
    prev: Option<Box<State>>,
    // key: Option<Box<dyn Any>>,
    // value: Option<Box<dyn FnOnce(&mut Context, &mut Constraints) -> Result<Box<dyn Any>, Error>>>,
    // opts: Vec<ConstraintsOpt>,
    // async: Option<AsyncState>,
}

impl State {
    
}
