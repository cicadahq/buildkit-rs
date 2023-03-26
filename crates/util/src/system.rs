pub enum OsFamily {
    Windows,
    Unix,
}

pub const DEFAULT_PATH_ENV_UNIX: &str =
    "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";

/// Windows style list of directories to search for executables. Each
/// directory is separated from the next by a colon `;` character.
pub const DEFAULT_PATH_ENV_WINDOWS: &str = "c:\\Windows\\System32;c:\\Windows";

pub const fn default_path_env(family: OsFamily) -> &'static str {
    match family {
        OsFamily::Windows => DEFAULT_PATH_ENV_WINDOWS,
        OsFamily::Unix => DEFAULT_PATH_ENV_UNIX,
    }
}
