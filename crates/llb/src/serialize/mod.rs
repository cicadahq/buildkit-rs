pub mod id;
pub mod node;

use buildkit_rs_proto::pb;
use prost::Message;

use crate::utils::OperationOutput;

use self::node::{Context, Node};

struct Constraints;

#[derive(Debug)]
pub struct Definition<'a> {
    input: OperationOutput<'a>,
    ignore_cache: bool,
}

impl<'a> Definition<'a> {
    pub fn new(input: OperationOutput<'a>) -> Self {
        Self {
            input,
            ignore_cache: false,
        }
    }
}

impl Definition<'_> {
    /// Convert to the protobuf representation
    pub fn into_pb(&self) -> pb::Definition {
        let mut ctx = Context::new();

        let final_node_iter = std::iter::once(self.serialize(&mut ctx).unwrap());

        let (def, metadata) = {
            ctx.into_registered_nodes()
                .chain(final_node_iter)
                .map(|node| {
                    (
                        node.bytes,
                        (
                            node.digest,
                            if self.ignore_cache {
                                pb::OpMetadata {
                                    ignore_cache: true,
                                    ..Default::default()
                                }
                            } else {
                                node.metadata
                            },
                        ),
                    )
                })
                .unzip()
        };

        pb::Definition {
            def,
            metadata,
            ..Default::default()
        }
    }

    pub fn with_ignore_cache(mut self, ignore_cache: bool) -> Self {
        self.ignore_cache = ignore_cache;
        self
    }

    fn serialize(&self, ctx: &mut Context) -> Option<Node> {
        let final_op = pb::Op {
            inputs: vec![pb::Input {
                digest: ctx.register(self.input.operation())?.digest.clone(),
                index: self.input.output().into(),
            }],

            ..Default::default()
        };

        Some(Node::new(final_op, Default::default()))
    }

    // Convert to the protobuf bytes representation
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.into_pb().encode(&mut buf).unwrap();
        buf
    }

    // Convert from the protobuf representation
    // fn from_definition(x: &pb::Definition) -> Self {
    //     let mut metadata = HashMap::new();
    //     for (k, v) in &x.metadata {
    //         metadata.insert(k.clone(), v.clone());
    //     }
    //     Self {
    //         def: x.def.clone(),
    //         metadata,
    //         source: x.source.clone(),
    //         constraints: None,
    //     }
    // }

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
