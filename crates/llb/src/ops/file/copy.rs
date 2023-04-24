use camino::Utf8PathBuf;

use crate::utils::OperationOutput;

use super::FileAction;

#[derive(Debug)]
pub(crate) struct Copy<'a> {
    src_path: Utf8PathBuf,
    src_input: OperationOutput<'a>,
    dst_path: Utf8PathBuf,
    dest_input: OperationOutput<'a>,
    // owner: Option<ChownOpt>,
    // mode: i32,
    // follow_symlink: bool,
    // dir_copy_contents: bool,
    // attempt_unpack_docker_compatibility: bool,
    // create_dest_path: bool,
    // allow_wildcard: bool,
    // allow_empty_wildcard: bool,
    // timestamp: i64,
    // include_patterns: Vec<String>,
    // exclude_patterns: Vec<String>,
}

impl<'a> Copy<'a> {
    pub fn new(
        src_path: impl Into<Utf8PathBuf>,
        src_input: OperationOutput<'a>,
        dst_path: impl Into<Utf8PathBuf>,
        dest_input: OperationOutput<'a>,
    ) -> Self {
        Self {
            src_path: src_path.into(),
            src_input,
            dst_path: dst_path.into(),
            dest_input,
        }
    }
}

impl<'a> From<Copy<'a>> for FileAction<'a> {
    fn from(copy: Copy<'a>) -> Self {
        Self::Copy(copy)
    }
}
