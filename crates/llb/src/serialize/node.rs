use buildkit_rs_proto::pb;
use prost::Message;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Debug};

use crate::sourcemap::SourceLocation;

use super::id::OperationId;

pub(crate) trait Operation: Debug + Send + Sync {
    fn id(&self) -> &OperationId;

    fn serialize(&self, cx: &mut Context) -> Option<Node>;
}

#[derive(Default)]
pub struct Context {
    inner: BTreeMap<u64, Node>,
}

impl Context {
    #[allow(clippy::map_entry)]
    pub(crate) fn register<'a>(&'a mut self, op: &dyn Operation) -> Option<&'a Node> {
        let id = **op.id();

        if !self.inner.contains_key(&id) {
            let node = op.serialize(self)?;
            self.inner.insert(id, node);
        }

        Some(self.inner.get(&id).unwrap())
    }

    #[cfg(test)]
    pub(crate) fn registered_nodes_iter(&self) -> impl Iterator<Item = &Node> {
        self.inner.iter().map(|pair| pair.1)
    }

    pub(crate) fn into_registered_nodes(self) -> impl Iterator<Item = Node> {
        self.inner.into_iter().map(|pair| pair.1)
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Node {
    pub bytes: Vec<u8>,
    pub digest: String,
    pub metadata: pb::OpMetadata,
    pub source_location: Option<SourceLocation>
}

impl Node {
    pub fn new(message: pb::Op, metadata: pb::OpMetadata) -> Self {
        let mut bytes = Vec::new();
        message.encode(&mut bytes).unwrap();

        Self {
            digest: Self::digest(&bytes),
            bytes,
            metadata,
            source_location: None
        }
    }

    pub fn digest(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let digest_bytes = hasher.finalize();
        format!("sha256:{digest_bytes:x}")
    }
}
