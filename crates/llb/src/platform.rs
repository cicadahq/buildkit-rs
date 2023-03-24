use std::borrow::Cow;

pub struct Platform<'a> {
    pub architecture: Cow<'a, str>,
    pub os: Cow<'a, str>,
    pub variant: Cow<'a, str>,
}

macro_rules! platform {
    ($name:ident, $os:expr, $arch:expr, $variant:expr) => {
        pub const $name: Platform = Platform {
            architecture: Cow::Borrowed($arch),
            os: Cow::Borrowed($os),
            variant: Cow::Borrowed($variant),
        };
    };
    ($name:ident, $val:expr) => {
        pub const $name: Platform = $val;
    };
}

platform!(LINUX_AMD64, "linux", "amd64", "");
platform!(LINUX_ARMHF, "linux", "arm", "v7");
platform!(LINUX_ARM, "linux", "arm", "v7");
platform!(LINUX_ARMEL, "linux", "arm", "v6");
platform!(LINUX_ARM64, "linux", "arm64", "");
platform!(LINUX_S390X, "linux", "s390x", "");
platform!(LINUX_PPC64, "linux", "ppc64", "");
platform!(LINUX_PPC64LE, "linux", "ppc64le", "");
platform!(DARWIN, "darwin", "amd64", "");
platform!(WINDOWS, "windows", "amd64", "");
