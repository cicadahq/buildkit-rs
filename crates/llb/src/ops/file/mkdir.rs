use camino::Utf8PathBuf;

use crate::utils::OperationOutput;

use super::FileAction;

#[derive(Debug)]
pub(crate) struct Mkdir<'a> {
    path: Utf8PathBuf,
    input: OperationOutput<'a>,

    make_parents: bool,
    // owner: Option<ChownOpt>,
    // mode: i32,
    // timestamp: i64,
}

impl<'a> Mkdir<'a> {
    pub fn new(path: impl Into<Utf8PathBuf>, input: OperationOutput<'a>) -> Self {
        Self {
            path: path.into(),
            input,
            make_parents: false,
        }
    }
}

impl<'a> From<Mkdir<'a>> for FileAction<'a> {
    fn from(mkdir: Mkdir<'a>) -> Self {
        Self::Mkdir(mkdir)
    }
}
