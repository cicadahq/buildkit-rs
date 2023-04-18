use camino::Utf8PathBuf;

enum FileAction {}

struct Copy {
    src: Utf8PathBuf,
    dst: Utf8PathBuf,
    // owner: Option<ChownOpt>,
    mode: i32,
    follow_symlink: bool,
    dir_copy_contents: bool,
    attempt_unpack_docker_compatibility: bool,
    create_dest_path: bool,
    allow_wildcard: bool,
    allow_empty_wildcard: bool,
    timestamp: i64,
    include_patterns: Vec<String>,
    exclude_patterns: Vec<String>,
}

// impl Copy {
//     pub fn new(
//         src_path: impl Into<Utf8PathBuf>,
//         src_input: OperationOutput<'a>,
//         dst_path: impl Into<Utf8PathBuf>,
//         dest_input: OperationOutput<'a>,
//     ) -> 
// }
