use std::{borrow::Cow, sync::Arc};

use buildkit_rs_proto::pb::{self, op::Op as OpEnum, ExecOp, Meta, NetMode, Op, SecurityMode};

use crate::{
    serialize::{
        id::OperationId,
        node::{Context, Node, Operation},
    },
    utils::{OperationOutput, OutputIdx},
    MultiBorrowedOutput, MultiOwnedOutput, OpMetadataBuilder,
};

use super::metadata::OpMetadata;

pub mod mount;

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
pub struct Exec<'a> {
    pub(crate) id: OperationId,
    pub(crate) metadata: OpMetadata,

    // pub proxy_env: Option<ProxyEnv>,
    pub context: Option<ExecContext>,
    pub mounts: Vec<mount::Mount<'a>>,
    // pub base: Option<State>,
    // pub constraints: Constraints,
    // pub is_validated: bool,
    // pub secrets: Vec<SecretInfo>,
    // pub ssh: Vec<SSHInfo>,
}

impl Exec<'static> {
    fn empty() -> Self {
        Self {
            id: OperationId::new(),
            metadata: OpMetadata::new(),
            context: None,
            mounts: vec![],
        }
    }

    pub fn new<I, S>(args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self {
            context: Some(ExecContext::new(
                args.into_iter().map(|s| s.as_ref().into()).collect(),
            )),
            ..Self::empty()
        }
    }

    pub fn shell(shell: impl AsRef<str>, args: impl AsRef<str>) -> Self {
        Self {
            context: Some(ExecContext::new(vec![
                shell.as_ref().into(),
                "-c".into(),
                args.as_ref().into(),
            ])),
            ..Self::empty()
        }
    }

    pub fn shlex(input: impl AsRef<str>) -> Self {
        let args = shlex::Shlex::new(input.as_ref()).collect();

        Self {
            context: Some(ExecContext::new(args)),
            ..Self::empty()
        }
    }
}

impl<'a> Exec<'a> {
    pub fn with_mount(mut self, mount: mount::Mount<'a>) -> Self {
        self.mounts.push(mount);
        self
    }

    pub fn with_env(mut self, env: Vec<String>) -> Self {
        self.context = Some(self.context.unwrap().with_env(env));
        self
    }

    pub fn with_cwd(mut self, cwd: String) -> Self {
        self.context = Some(self.context.unwrap().with_cwd(cwd));
        self
    }
}

#[derive(Debug, Clone)]
pub struct ExecContext {
    pub args: Vec<String>,
    pub env: Vec<String>,
    pub cwd: Cow<'static, str>,
    pub user: Cow<'static, str>,
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
        self.cwd = cwd.into();
        self
    }

    pub fn with_user(mut self, user: String) -> Self {
        self.user = user.into();
        self
    }
}

impl Operation for Exec<'_> {
    fn id(&self) -> &OperationId {
        &self.id
    }

    fn serialize(&self, ctx: &mut Context) -> Option<Node> {
        let mut mounts: Vec<pb::Mount> = vec![];
        let mut inputs: Vec<pb::Input> = vec![];

        let mut input_index = 0;
        for mount in &self.mounts {
            let input_index = if let Some(input) = mount.input() {
                let current_index = input_index;
                let node = ctx.register(input.operation()).unwrap();

                inputs.push(pb::Input {
                    digest: node.digest.clone(),
                    index: input.output().into(),
                });

                input_index += 1;

                current_index
            } else {
                -1
            };

            mounts.push(mount.to_pb(input_index));
        }

        let meta = self.context.as_ref().map(|ctx| Meta {
            args: ctx.args.clone(),
            env: ctx.env.clone(),
            cwd: ctx.cwd.clone().into_owned(),
            user: ctx.user.clone().into_owned(),
            ..Default::default()
        });

        let exec_op = ExecOp {
            meta,
            mounts,
            network: NetMode::Unset.into(),
            security: SecurityMode::Sandbox.into(),
            secretenv: vec![],
        };

        Some(Node::new(
            Op {
                op: Some(OpEnum::Exec(exec_op)),
                inputs,
                ..Default::default()
            },
            self.metadata.clone().into(),
        ))
    }
}

impl OpMetadataBuilder for Exec<'_> {
    fn metadata(&self) -> &OpMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut OpMetadata {
        &mut self.metadata
    }
}

impl<'a, 'b: 'a> MultiBorrowedOutput<'b> for Exec<'b> {
    fn output(&'b self, index: u32) -> OperationOutput<'b> {
        // TODO: check if the requested index available.
        OperationOutput::borrowed(self, OutputIdx(index))
    }
}

impl<'a> MultiOwnedOutput<'a> for Arc<Exec<'a>> {
    fn output(&self, index: u32) -> OperationOutput<'a> {
        // TODO: check if the requested index available.
        OperationOutput::owned(self.clone(), OutputIdx(index))
    }
}
