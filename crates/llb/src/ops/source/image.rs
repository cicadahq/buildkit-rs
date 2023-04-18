use std::{collections::HashMap, sync::Arc};

use buildkit_rs_proto::pb::{self, op::Op as OpEnum, Op};
use buildkit_rs_reference::Reference;

use crate::{
    ops::{metadata::{attr::Attr, OpMetadata, OpMetadataBuilder}, output::{SingleOwnedOutput, SingleBorrowedOutput}},
    serialize::{
        id::OperationId,
        node::{Context, Node, Operation},
    }, utils::{OutputIdx, OperationOutput},
};

#[derive(Debug, Clone, Copy, Default)]
pub enum ResolveMode {
    #[default]
    Default,
    Pull,
    Local,
}

impl ResolveMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResolveMode::Default => "default",
            ResolveMode::Pull => "pull",
            ResolveMode::Local => "local",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    id: OperationId,
    metadata: OpMetadata,

    reference: Reference,
    resolve_mode: Option<ResolveMode>,
}

impl Image {
    pub fn new(name: impl AsRef<str>) -> Self {
        let reference = Reference::parse_normalized_named(name.as_ref()).unwrap();

        Self {
            id: OperationId::new(),
            metadata: OpMetadata::new(),
            reference,
            resolve_mode: None,
        }
    }

    pub fn with_resolve_mode(mut self, mode: ResolveMode) -> Self {
        self.resolve_mode = Some(mode);
        self
    }
}

impl Operation for Image {
    fn id(&self) -> &OperationId {
        &self.id
    }

    fn serialize(&self, _: &mut Context) -> Option<Node> {
        let mut attrs = HashMap::default();

        if let Some(ref mode) = self.resolve_mode {
            attrs.insert(Attr::IMAGE_RESOLVE_MODE.into(), mode.as_str().into());
        }

        Some(Node::new(
            Op {
                op: Some(OpEnum::Source(pb::SourceOp {
                    // Should we use docker-image:// or one of the other variants (container-image://)
                    identifier: format!("docker-image://{}", self.reference),
                    attrs,
                })),

                ..Default::default()
            },
            self.metadata.clone().into(),
        ))
    }
}

impl OpMetadataBuilder for Image {
    fn metadata(&self) -> &OpMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut OpMetadata {
        &mut self.metadata
    }
}

impl<'a> SingleBorrowedOutput<'a> for Image {
    fn output(&'a self) -> OperationOutput<'a> {
        OperationOutput::borrowed(self, OutputIdx(0))
    }
}

impl<'a> SingleOwnedOutput<'static> for Arc<Image> {
    fn output(&self) -> OperationOutput<'static> {
        OperationOutput::owned(self.clone(), OutputIdx(0))
    }
}