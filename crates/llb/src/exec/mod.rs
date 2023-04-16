use buildkit_rs_proto::pb::{
    self, op::Op as OpEnum, ExecOp, Meta, NetMode, Op, SecretEnv, SecurityMode,
};

use crate::{
    op_metadata::OpMetadata,
    serialize::{
        id::OperationId,
        node::{Context, Node, Operation},
    },
};

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

#[derive(Debug, Clone)]
pub struct Exec {
    pub(crate) id: OperationId,
    pub(crate) metadata: OpMetadata,

    // pub proxy_env: Option<ProxyEnv>,
    pub context: Option<ExecContext>,
    pub mounts: Vec<mount::Mount>,
    // pub base: Option<State>,
    // pub constraints: Constraints,
    // pub is_validated: bool,
    // pub secrets: Vec<SecretInfo>,
    // pub ssh: Vec<SSHInfo>,
}

impl Exec {
    pub fn new() -> Self {
        Self {
            id: OperationId::new(),
            metadata: OpMetadata::new(),

            context: None,
            mounts: vec![],
        }
    }

    pub fn shlex(input: impl AsRef<str>) -> Self {
        let args = shlex::Shlex::new(input.as_ref()).into_iter().collect();

        Self {
            context: Some(ExecContext::new(args)),
            ..Self::new()
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

impl Operation for Exec {
    fn id(&self) -> &OperationId {
        &self.id
    }

    fn serialize(&self, cx: &mut Context) -> Option<Node> {
        let meta = self.context.as_ref().map(|ctx| {
            let mut meta = Meta::default();
            meta.args = ctx.args.clone();
            meta.env = ctx.env.clone();
            meta.cwd = ctx.cwd.clone();
            meta.user = ctx.user.clone();
            meta
        });

        let exec_op = ExecOp {
            meta,
            mounts: vec![],
            network: NetMode::None.into(),
            security: SecurityMode::Sandbox.into(),
            secretenv: vec![],
        };

        Some(Node::new(
            Op {
                op: Some(OpEnum::Exec(exec_op)),
                ..Default::default()
            },
            self.metadata.clone().into(),
        ))
    }
}
