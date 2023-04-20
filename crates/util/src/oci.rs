use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OciBackend {
    #[default]
    Docker,
    Podman,
}

impl OciBackend {
    pub fn as_str(&self) -> &'static str {
        match self {
            OciBackend::Docker => "docker",
            OciBackend::Podman => "podman",
        }
    }
}

impl fmt::Display for OciBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
pub struct OciBackendFromStrError(String);

impl std::error::Error for OciBackendFromStrError {}

impl fmt::Display for OciBackendFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown OCI backend: {}", self.0)
    }
}

impl FromStr for OciBackend {
    type Err = OciBackendFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "docker" => Ok(OciBackend::Docker),
            "podman" => Ok(OciBackend::Podman),
            _ => Err(OciBackendFromStrError(s.to_owned())),
        }
    }
}

impl AsRef<str> for OciBackend {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
