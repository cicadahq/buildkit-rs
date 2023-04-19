use path_clean::clean;
use std::io::{BufRead, BufReader, Read};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid pattern '{0}'")]
    InvalidPattern(String),
    #[error("non-UTF8 pattern")]
    NonUtf8Pattern,
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// Read all reads an ignore file and returns the list of file patterns
/// to ignore. Note this will trim whitespace from each line as well
/// as use Rust's `PathBuf` to get the cleanest path for each.
///
/// Based on the implementation here: https://github.com/moby/buildkit/blob/1077362ebe0fc7e8f9c2634a49e07733a63ea1c9/frontend/dockerfile/dockerignore/dockerignore.go
pub fn read_ignore_to_list<R: Read>(reader: R) -> Result<Vec<String>, Error> {
    let mut excludes = Vec::new();

    let mut buf_reader = BufReader::new(reader);
    let utf8_bom = [0xEF, 0xBB, 0xBF];
    let mut current_line = 0;

    let mut line = String::new();
    while buf_reader.read_line(&mut line)? > 0 {
        // We trim utf8 bom from the first line
        if current_line == 0 {
            line = line
                .trim_start_matches(|c: char| utf8_bom.contains(&(c as u8)))
                .to_string();
        }
        current_line += 1;
        // Lines starting with # (comments) are ignored before processing
        if line.starts_with('#') {
            line.clear();
            continue;
        }
        let mut pattern = line.trim().to_string();
        if pattern.is_empty() {
            line.clear();
            continue;
        }
        // normalize absolute paths to paths relative to the context
        // (taking care of '!' prefix)
        let invert = pattern.starts_with('!');
        if invert {
            pattern = pattern[1..].trim().to_string();
        }

        if !pattern.is_empty() {
            pattern = clean(&pattern)
                .to_str()
                .ok_or_else(|| Error::NonUtf8Pattern)?
                .to_owned();
            if pattern.starts_with('/') {
                pattern = pattern[1..].to_string();
            }
        }

        if invert {
            pattern = format!("!{}", pattern);
        }

        excludes.push(pattern);
        line.clear();
    }

    Ok(excludes)
}
