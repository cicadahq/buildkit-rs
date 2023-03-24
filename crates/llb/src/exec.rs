pub struct Exec {
    pub context: Option<ExecContext>,
    // pub mounts: Vec<Mount>,
    // pub network: i32,
    // pub security: i32,
    // pub secretenv: Vec<SecretEnv>,
}

impl Exec {
    pub fn new() -> Self {
        Self { context: None }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExecContext {
    pub args: Vec<String>,
    pub env: Vec<String>,
    pub cwd: String,
    pub user: String,
}

impl ExecContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn env(mut self, env: Vec<String>) -> Self {
        self.env = env;
        self
    }

    pub fn cwd(mut self, cwd: String) -> Self {
        self.cwd = cwd;
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = user;
        self
    }
}

pub fn shlex(input: impl AsRef<str>) -> Exec {
    let args = shlex::Shlex::new(input.as_ref()).into_iter().collect();

    Exec {
        context: Some(ExecContext {
            args,
            ..Default::default()
        }),
    }
}
