use std::collections::HashMap;

use buildkit_rs_proto::pb::{self, op::Op as OpEnum, Op};
use buildkit_rs_reference::Reference;

use crate::{
    op_metadata::{attr::Attr, OpMetadata, OpMetadataBuilder},
    serialize::{
        id::OperationId,
        node::{Context, Node, Operation},
    },
};

#[derive(Debug, Clone)]
pub struct Image {
    id: OperationId,
    metadata: OpMetadata,

    name: String,

    exclude: Vec<String>,
    include: Vec<String>,
}

impl Image {
    pub fn new(name: impl AsRef<str>) -> Self {
        let normalized_name = Reference::parse_normalized_named(name.as_ref())
            .expect("failed to parse image name")
            .to_string();

        Self {
            id: OperationId::new(),
            metadata: OpMetadata::new(),
            name: normalized_name,
            exclude: Vec::new(),
            include: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_include<I, S>(mut self, include: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.include = include.into_iter().map(|s| s.as_ref().into()).collect();
        self
    }

    pub fn with_exclude<I, S>(mut self, exclude: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.exclude = exclude.into_iter().map(|s| s.as_ref().into()).collect();
        self
    }
}

impl Operation for Image {
    fn id(&self) -> &OperationId {
        &self.id
    }

    fn serialize(&self, cx: &mut Context) -> Option<Node> {
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
                    identifier: self.name.clone(),
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
