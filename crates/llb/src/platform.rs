use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Platform<'a> {
    /// The architecture of the platform
    pub architecture: Cow<'a, str>,
    /// The name of the operating system
    pub os: Cow<'a, str>,
    /// The variant of the architecture
    pub variant: Option<Cow<'a, str>>,
}

impl<'a> Platform<'a> {
    pub const fn new(os: &'a str, arch: &'a str, variant: Option<&'a str>) -> Self {
        Self {
            architecture: Cow::Borrowed(arch),
            os: Cow::Borrowed(os),
            // We have to manually map since `map` is not const yet
            // TODO: Replace with `map` when it is const
            variant: match variant {
                Some(variant) => Some(Cow::Borrowed(variant)),
                None => None,
            },
        }
    }
}

impl Platform<'_> {
    pub fn into_static(self) -> Platform<'static> {
        Platform {
            architecture: Cow::Owned(self.architecture.into_owned()),
            os: Cow::Owned(self.os.into_owned()),
            variant: match self.variant {
                Some(variant) => Some(Cow::Owned(variant.into_owned())),
                None => None,
            },
        }
    }
}

impl fmt::Display for Platform<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.os, self.architecture)?;
        if let Some(variant) = &self.variant {
            write!(f, "-{}", variant)?;
        }
        Ok(())
    }
}

pub const LINUX_AMD64: Platform = Platform::new("linux", "amd64", None);
pub const LINUX_ARMHF: Platform = Platform::new("linux", "arm", Some("v7"));
pub const LINUX_ARM: Platform = LINUX_ARMHF; // ALIAS FOR LINUX_ARMHF
pub const LINUX_ARMEL: Platform = Platform::new("linux", "arm", Some("v6"));
pub const LINUX_ARM64: Platform = Platform::new("linux", "arm64", None);
pub const LINUX_S390X: Platform = Platform::new("linux", "s390x", None);
pub const LINUX_PPC64: Platform = Platform::new("linux", "ppc64", None);
pub const LINUX_PPC64LE: Platform = Platform::new("linux", "ppc64le", None);
pub const DARWIN: Platform = Platform::new("darwin", "amd64", None);
pub const WINDOWS: Platform = Platform::new("windows", "amd64", None);
