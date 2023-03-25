pub mod attr;
mod cap;

use std::collections::HashMap;

use attr::Attr;
use buildkit_rs_proto::pb;

#[derive(Debug, Clone)]
pub struct OpMetadata {
    pub ignore_cache: bool,
    pub description: HashMap<Attr, String>,
}

impl OpMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for OpMetadata {
    fn default() -> Self {
        Self {
            ignore_cache: false,
            description: HashMap::new(),
        }
    }
}

impl Into<pb::OpMetadata> for OpMetadata {
    fn into(self) -> pb::OpMetadata {
        pb::OpMetadata {
            ignore_cache: self.ignore_cache,
            description: self
                .description
                .into_iter()
                .map(|(k, v)| (k.into(), v))
                .collect(),
            caps: HashMap::new(),
            export_cache: None,
            progress_group: None,
        }
    }
}

pub trait OpMetadataBuilder: Sized {
    fn metadata(&self) -> &OpMetadata;
    fn metadata_mut(&mut self) -> &mut OpMetadata;

    fn ignore_cache(mut self, ignore: bool) -> Self {
        self.metadata_mut().ignore_cache = ignore;
        self
    }

    fn set_description(mut self, attr: Attr, value: impl AsRef<str>) -> Self {
        self.metadata_mut()
            .description
            .insert(attr, value.as_ref().to_owned());
        self
    }

    fn remove_description(mut self, attr: Attr) -> Self {
        self.metadata_mut().description.remove(&attr);
        self
    }

    fn set_custom_name(self, name: impl AsRef<str>) -> Self {
        self.set_description(Attr::CUSTOM_NAME, name)
    }
}
