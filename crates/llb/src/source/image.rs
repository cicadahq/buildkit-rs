use std::collections::HashMap;

use buildkit_rs_proto::pb;

use crate::serialize::{
    id::OperationId,
    node::{Context, Node, Operation},
};

#[derive(Debug, Clone)]
struct Image {
    id: OperationId,
    name: String,
    description: HashMap<String, String>,
    ignore_cache: bool,

    exclude: Vec<String>,
    include: Vec<String>,
}

impl Image {
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: OperationId::new(),
            description: HashMap::default(),
            ignore_cache: false,
            exclude: vec![],
            include: vec![],
        }
    }

    pub fn include<I, S>(mut self, include: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.include = include.into_iter().map(|s| s.as_ref().into()).collect();
        self
    }

    pub fn exclude<I, S>(mut self, exclude: I) -> Self
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
                "local.excludepatterns".into(),
                serde_json::to_string(&self.exclude).unwrap(),
            );
        }

        if !self.include.is_empty() {
            attrs.insert(
                "local.includepattern".into(),
                serde_json::to_string(&self.include).unwrap(),
            );
        }

        Some(Node::new(
            pb::Op {
                op: Some(pb::op::Op::Source(pb::SourceOp {
                    identifier: self.name.clone(),
                    attrs,
                })),
                ..Default::default()
            },
            pb::OpMetadata {
                description: self.description.clone(),
                ignore_cache: self.ignore_cache,
                ..Default::default()
            },
        ))
    }
}
