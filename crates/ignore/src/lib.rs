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
/// Based on the implementation here, we ignore the bom stripping however:
/// <https://github.com/moby/buildkit/blob/1077362ebe0fc7e8f9c2634a49e07733a63ea1c9/frontend/dockerfile/dockerignore/dockerignore.go>
pub fn read_ignore_to_list<R: Read>(reader: R) -> Result<Vec<String>, Error> {
    let mut excludes = Vec::new();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    while buf_reader.read_line(&mut line)? > 0 {
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_empty_reader() {
        let input = "";
        let reader = Cursor::new(input);
        let result = read_ignore_to_list(reader).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_ignore_file() {
        let input = r#"
# This is a comment
*.tmp

file.txt
    "#;
        let reader = Cursor::new(input);
        let result = read_ignore_to_list(reader).unwrap();
        assert_eq!(result, vec!["*.tmp", "file.txt"]);
    }

    #[test]
    fn test_ignore_with_inverted_pattern() {
        let input = ["!file.txt", "*.tmp", "!./file.txt"].join("\n");
        let reader = Cursor::new(input);
        let result = read_ignore_to_list(reader).unwrap();
        assert_eq!(result, vec!["!file.txt", "*.tmp", "!file.txt"]);
    }
}
