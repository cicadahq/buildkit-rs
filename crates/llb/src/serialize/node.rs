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
    pub(crate) fn new() -> Self {
        Self::default()
    }

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

fn digest(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest_bytes = hasher.finalize();
    format!("sha256:{digest_bytes:x}")
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Node {
    pub bytes: Vec<u8>,
    pub digest: String,
    pub metadata: pb::OpMetadata,
    pub source_location: Option<SourceLocation>,
}

impl Node {
    pub fn new(message: pb::Op, metadata: pb::OpMetadata) -> Self {
        let mut bytes = Vec::new();
        message.encode(&mut bytes).unwrap();

        Self {
            digest: digest(&bytes),
            bytes,
            metadata,
            source_location: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digest() {
        let bytes = b"hello world";
        let digest = digest(bytes);

        assert_eq!(
            digest,
            "sha256:b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }
}
