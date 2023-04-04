use buildkit_rs_proto::pb;

use crate::sourcemap::SourceLocation;
use crate::state::Constraints;
use crate::Digest;

pub struct MarshalCache {
    digest: Digest,
    dt: Vec<u8>,
    md: pb::OpMetadata,
    srcs: Vec<SourceLocation>,
    constraints: Constraints,
}
