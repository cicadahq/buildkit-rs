pub mod exec;
pub mod image;
pub mod platform;
use std::collections::HashMap;

use buildkit_rs_proto::pb;

pub fn scratch() {
    pb::Definition {
        source: Some(pb::Source {
            locations: HashMap::new(),
            infos: vec![],
        }),
        ..Default::default()
    };
}
