pub mod attr;
pub mod cap;

use std::collections::HashMap;

use attr::Attr;
use buildkit_rs_proto::pb;

#[derive(Debug, Clone, Default)]
pub struct OpMetadata {
    pub ignore_cache: bool,
    pub description: HashMap<Attr, String>,
}

impl OpMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<OpMetadata> for pb::OpMetadata {
    fn from(val: OpMetadata) -> Self {
        pb::OpMetadata {
            ignore_cache: val.ignore_cache,
            description: val
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

    fn with_description(mut self, attr: Attr, value: impl AsRef<str>) -> Self {
        self.metadata_mut()
            .description
            .insert(attr, value.as_ref().to_owned());
        self
    }

    fn remove_description(mut self, attr: Attr) -> Self {
        self.metadata_mut().description.remove(&attr);
        self
    }

    fn with_custom_name(self, name: impl AsRef<str>) -> Self {
        self.with_description(Attr::CUSTOM_NAME, name)
    }
}
