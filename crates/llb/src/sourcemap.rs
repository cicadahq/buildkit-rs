/*type SourceLocation struct {
    SourceMap *SourceMap
    Ranges    []*pb.Range
}

type SourceMap struct {
    State      *State
    Definition *Definition
    Filename   string
    Data       []byte
}
*/

use buildkit_rs_proto::pb;

#[derive(Debug, Clone)]
pub struct SourceLocation {
    source_map: SourceMap,
    ranges: Vec<pb::Range>,
}

#[derive(Debug, Clone)]
struct SourceMap {
    pub filename: String,
    pub data: Vec<u8>,
}
