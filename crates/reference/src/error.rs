use thiserror::Error;

use crate::consts::NAME_TOTAL_LENGTH_MAX;

/// The error type for parsing a reference
#[derive(Debug, Clone, Error)]
pub enum Error {
    /// The reference is not valid, this is the most generic error
    #[error("invalid reference format")]
    InvalidReferenceFormat,

    /// The repository name is not valid as it contains uppercase characters
    #[error("repository name must be lowercase")]
    NameContainsUppercase,

    /// The name is empty
    #[error("repository name must have at least one component")]
    NameEmpty,

    /// The name is too long
    #[error("repository name must not be more than {NAME_TOTAL_LENGTH_MAX} characters")]
    NameTooLong,

    /// The name matches the identifier pattern and is therefore not allowed
    #[error("invalid repository name, cannot specify 64-byte hexadecimal strings")]
    NameIdentifier,
}
