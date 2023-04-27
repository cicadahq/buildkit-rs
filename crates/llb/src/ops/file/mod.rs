mod copy;
mod mkdir;

use std::sync::Arc;

use buildkit_rs_proto::pb::{op::Op as OpEnum, FileOp, Op};
use copy::Copy;
use mkdir::Mkdir;

use crate::{
    serialize::{
        id::OperationId,
        node::{Context, Node, Operation},
    },
    utils::{OperationOutput, OutputIdx},
    MultiBorrowedOutput, MultiOwnedOutput, OpMetadataBuilder,
};

use super::metadata::OpMetadata;

#[derive(Debug)]
enum FileAction<'a> {
    Copy(Copy<'a>),
    Mkdir(Mkdir<'a>),
}

#[derive(Debug)]
struct FileActions<'a> {
    id: OperationId,
    metadata: OpMetadata,

    actions: Vec<FileAction<'a>>,
}

impl FileActions<'_> {
    pub fn new() -> Self {
        Self {
            id: OperationId::new(),
            metadata: OpMetadata::new(),
            actions: Vec::new(),
        }
    }
}

impl<'a> FileActions<'a> {
    pub fn with_action(mut self, action: impl Into<FileAction<'a>>) -> Self {
        self.actions.push(action.into());
        self
    }
}

impl<'a, 'b: 'a> MultiBorrowedOutput<'b> for FileActions<'b> {
    fn output(&'b self, index: u32) -> OperationOutput<'b> {
        // TODO: check if the requested index available.
        OperationOutput::borrowed(self, OutputIdx(index))
    }
}

impl<'a> MultiOwnedOutput<'a> for Arc<FileActions<'a>> {
    fn output(&self, index: u32) -> OperationOutput<'a> {
        // TODO: check if the requested index available.
        OperationOutput::owned(self.clone(), OutputIdx(index))
    }
}

impl Operation for FileActions<'_> {
    fn id(&self) -> &OperationId {
        &self.id
    }

    fn serialize(&self, _cx: &mut Context) -> Option<Node> {
        let actions = vec![];

        Some(Node::new(
            Op {
                op: Some(OpEnum::File(FileOp { actions })),

                ..Default::default()
            },
            self.metadata.clone().into(),
        ))
    }
}

impl OpMetadataBuilder for FileActions<'_> {
    fn metadata(&self) -> &OpMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut OpMetadata {
        &mut self.metadata
    }
}
