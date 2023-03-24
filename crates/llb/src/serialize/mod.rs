pub mod id;
pub mod node;

use buildkit_rs_proto::pb;
use prost::Message;
use std::collections::HashMap;

type Digest = String;

struct Constraints;

struct Definition {
    def: Vec<Vec<u8>>,
    metadata: HashMap<Digest, pb::OpMetadata>,
    source: Option<pb::Source>,
    constraints: Option<Constraints>,
}

impl Definition {
    /// Convert to the protobuf representation
    fn into_definition(&self) -> pb::Definition {
        let mut md = HashMap::new();
        for (k, v) in &self.metadata {
            md.insert(k.clone(), v.clone());
        }
        pb::Definition {
            def: self.def.clone(),
            source: self.source.clone(),
            metadata: md,
        }
    }

    // Convert to the protobuf bytes representation
    fn into_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.into_definition().encode(&mut buf).unwrap();
        buf
    }

    /// Convert from the protobuf representation
    fn from_definition(x: &pb::Definition) -> Self {
        let mut metadata = HashMap::new();
        for (k, v) in &x.metadata {
            metadata.insert(k.clone(), v.clone());
        }
        Self {
            def: x.def.clone(),
            metadata,
            source: x.source.clone(),
            constraints: None,
        }
    }

    // fn head(&self) -> Option<Digest> {
    //     if self.def.is_empty() {
    //         return None;
    //     }

    //     let last = self.def.last().unwrap();

    //     let mut pop = pb::Op::new();
    //     if let Err(_) = pop.merge_from_bytes(last) {
    //         return None;
    //     }

    //     if pop.inputs.is_empty() {
    //         return None;
    //     }

    //     Some(pop.inputs[0].digest.clone())
    // }
}
