use std::{collections::HashMap, sync::Arc};

use buildkit_rs_proto::pb::{self, op::Op as OpEnum, Op};

use crate::{
    ops::{
        metadata::{attr::Attr, OpMetadata, OpMetadataBuilder},
        output::{SingleBorrowedOutput, SingleOwnedOutput},
    },
    serialize::{
        id::OperationId,
        node::{Context, Node, Operation},
    },
    utils::{OperationOutput, OutputIdx},
};

#[derive(Debug, Clone)]
pub struct Local {
    id: OperationId,
    metadata: OpMetadata,

    name: String,

    exclude: Vec<String>,
    include: Vec<String>,
}

impl Local {
    pub fn new(name: String) -> Self {
        Self {
            id: OperationId::new(),
            metadata: OpMetadata::new(),
            name,
            exclude: Vec::new(),
            include: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_includes<I, S>(mut self, include: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.include = include.into_iter().map(|s| s.as_ref().into()).collect();
        self
    }

    pub fn with_include(mut self, include: impl AsRef<str>) -> Self {
        self.include.push(include.as_ref().into());
        self
    }

    pub fn with_excludes<I, S>(mut self, exclude: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.exclude = exclude.into_iter().map(|s| s.as_ref().into()).collect();
        self
    }

    pub fn with_exclude(mut self, exclude: impl AsRef<str>) -> Self {
        self.exclude.push(exclude.as_ref().into());
        self
    }
}

impl Operation for Local {
    fn id(&self) -> &OperationId {
        &self.id
    }

    fn serialize(&self, _: &mut Context) -> Option<Node> {
        let mut attrs = HashMap::default();

        if !self.exclude.is_empty() {
            attrs.insert(
                Attr::EXCLUDE_PATTERNS.into(),
                serde_json::to_string(&self.exclude).unwrap(),
            );
        }

        if !self.include.is_empty() {
            attrs.insert(
                Attr::INCLUDE_PATTERNS.into(),
                serde_json::to_string(&self.include).unwrap(),
            );
        }

        Some(Node::new(
            Op {
                op: Some(OpEnum::Source(pb::SourceOp {
                    identifier: format!("local://{}", self.name),
                    attrs,
                })),

                ..Default::default()
            },
            self.metadata.clone().into(),
        ))
    }
}

impl OpMetadataBuilder for Local {
    fn metadata(&self) -> &OpMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut OpMetadata {
        &mut self.metadata
    }
}

impl<'a> SingleBorrowedOutput<'a> for Local {
    fn output(&'a self) -> OperationOutput<'a> {
        OperationOutput::borrowed(self, OutputIdx(0))
    }
}

impl SingleOwnedOutput<'static> for Arc<Local> {
    fn output(&self) -> OperationOutput<'static> {
        OperationOutput::owned(self.clone(), OutputIdx(0))
    }
}
