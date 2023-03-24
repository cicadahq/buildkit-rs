mod mount;

/*
type ExecOp struct {
    proxyEnv    *ProxyEnv
    root        Output
    mounts      []*mount
    base        State
    constraints Constraints
    isValidated bool
    secrets     []SecretInfo
    ssh         []SSHInfo
}
*/

pub struct Exec {
    // pub proxy_env: Option<ProxyEnv>,
    pub context: Option<ExecContext>,
    // pub mounts: Vec<mount::Mount>,
    // pub base: Option<State>,
    // pub constraints: Constraints,
    // pub is_validated: bool,
    // pub secrets: Vec<SecretInfo>,
    // pub ssh: Vec<SSHInfo>,
}

impl Exec {
    pub fn new() -> Self {
        Self { context: None }
    }

    pub fn shlex(input: impl AsRef<str>) -> Self {
        let args = shlex::Shlex::new(input.as_ref()).into_iter().collect();
    
        Self {
            context: Some(ExecContext::new(args)),
        }
    }
}


#[derive(Debug, Clone)]
pub struct ExecContext {
    pub args: Vec<String>,
    pub env: Vec<String>,
    pub cwd: String,
    pub user: String,
}

impl ExecContext {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args,
            env: vec![],
            cwd: "/".into(),
            user: "root".into(),
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn with_env(mut self, env: Vec<String>) -> Self {
        self.env = env;
        self
    }

    pub fn with_cwd(mut self, cwd: String) -> Self {
        self.cwd = cwd;
        self
    }

    pub fn with_user(mut self, user: String) -> Self {
        self.user = user;
        self
    }
}


