use std::collections::HashMap;

use buildkit_rs_proto::pb;

use crate::{
    // op_metadata::{attr::Attr, OpMetadata, OpMetadataBuilder},
    serialize::{
        id::OperationId,
        node::{Context, Node, Operation},
    },
};

#[derive(Debug, Clone)]
pub struct Image {
    id: OperationId,
    metadata: pb::OpMetadata,

    name: String,

    exclude: Vec<String>,
    include: Vec<String>,
}

impl Image {
    pub fn new(name: String) -> Self {
        // TODO: parse the name properly
        Self {
            id: OperationId::new(),
            name,
            metadata:pb::OpMetadata::default(),
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

// impl Operation for Image {
//     fn id(&self) -> &OperationId {
//         &self.id
//     }
// 
//     fn serialize(&self) -> Option<Node> {
//         let mut attrs = HashMap::default();
// 
//         if !self.exclude.is_empty() {
//             attrs.insert(
//                 Attr::EXCLUDE_PATTERNS.into(),
//                 serde_json::to_string(&self.exclude).unwrap(),
//             );
//         }
// 
//         if !self.include.is_empty() {
//             attrs.insert(
//                 Attr::INCLUDE_PATTERNS.into(),
//                 serde_json::to_string(&self.include).unwrap(),
//             );
//         }
// 
//         Some(Node::new(
//             pb::Op {
//                 op: Some(pb::op::Op::Source(pb::SourceOp {
//                     identifier: self.name.clone(),
//                     attrs,
//                 })),
//                 ..Default::default()
//             },
//             self.metadata.clone().into(),
//         ))
//     }
// }
