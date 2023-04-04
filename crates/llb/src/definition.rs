use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use buildkit_rs_proto::pb;

use crate::marshal::MarshalCache;
use crate::sourcemap::SourceLocation;
use crate::Digest;

pub struct DefinitionOp {
    marshal_cache: Arc<Mutex<MarshalCache>>,
    ops: Arc<Mutex<HashMap<Digest, pb::Op>>>,
    defs: Arc<Mutex<HashMap<Digest, Vec<u8>>>>,
    metas: Arc<Mutex<HashMap<Digest, pb::OpMetadata>>>,
    sources: Arc<Mutex<HashMap<Digest, Vec<SourceLocation>>>>,
    platforms: Arc<Mutex<HashMap<Digest, Option<pb::Platform>>>>,
    dgst: Digest,
    index: pb::OutputIndex,
    input_cache: Arc<Mutex<HashMap<Digest, Vec<Option<Arc<DefinitionOp>>>>>>,
}
