#![allow(unused)]

// https://github.com/distribution/distribution/blob/e5d5810851d1f17a5070e9b6f940d8af98ea3c29/reference/regexp.go

// Primitve components

use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

/// optional wraps the expression in a non-capturing group and makes the
/// production optional.
macro_rules! optional {
    ($( $x:expr ),*) => {
        {
            let mut s = String::new();
            s.push_str("(?:");
            $(
                s.push_str($x);
            )*
            s.push_str(")?");
            s
        }
    };
}

/// anyTimes wraps the expression in a non-capturing group that can occur
/// any number of times.
fn any_times(res: &str) -> String {
    format!("(?:{res})*")
}

/// capture wraps the expression in a capturing group.
fn capture(res: &str) -> String {
    format!("({res})")
}

/// Make an anchored expression by adding ^ and $ to the beginning and end
macro_rules! anchored {
    ($( $x:expr ),*) => {
        {
            let mut s = String::new();
            s.push('^');
            $(
                s.push_str($x);
            )*
            s.push('$');
            s
        }
    };
}

// Main regexs

/// DIGEST_REGEX matches well-formed digests, including algorithm (e.g. "sha256:<encoded>").
static DIGEST_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(DIGEST_PAT).unwrap());

/// DOMAIN_REGEX matches hostname or IP-addresses, optionally including a port
/// number. It defines the structure of potential domain components that may be
/// part of image names. This is purposely a subset of what is allowed by DNS to
/// ensure backwards compatibility with Docker image names. It may be a subset of
/// DNS domain name, an IPv4 address in decimal format, or an IPv6 address between
/// square brackets (excluding zone identifiers as defined by [RFC 6874] or special
/// addresses such as IPv4-Mapped).
///
/// [RFC 6874]: https://www.rfc-editor.org/rfc/rfc6874.
static DOMAIN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(&domain_and_port()).unwrap());

/// IDENTIFIER_REGEX is the format for string identifier used as a
/// content addressable identifier using sha256. These identifiers
/// are like digests without the algorithm, since sha256 is used.
static IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(IDENTIFIER).unwrap());

/// NAME_REGEX is the format for the name component of references, including
/// an optional domain and port, but without tag or digest suffix.
static NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(&name_pat()).unwrap());

/// ReferenceRegexp is the full supported format of a reference. The regexp
/// is anchored and has capturing groups for name, tag, and digest
/// components.
pub(crate) static REFERENCE_REGEX: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(&reference_pat())
        .size_limit(100_000_000)
        .build()
        .unwrap()
});

/// TagRegexp matches valid tag names. From [docker/docker:graph/tags.go](https://github.com/moby/moby/blob/v1.6.0/graph/tags.go#L26-L28)
static TAG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(TAG).unwrap());

/// ANCHORED_NAME_REGEXP is used to parse a name value, capturing the
/// domain and trailing components.
pub(crate) static ANCHORED_NAME_REGEXP: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&anchored!(
        &optional!(&capture(&domain_and_port()), "/"),
        &capture(&remote_name())
    ))
    .unwrap()
});

/// ANCHORED_IDENTIFIER_REGEXP is used to check or match an
/// identifier value, anchored at start and end of string.
pub(crate) static ANCHORED_IDENTIFIER_REGEXP: Lazy<Regex> =
    Lazy::new(|| Regex::new(&anchored!(IDENTIFIER)).unwrap());

/// ANCHORED_TAG_REGEXP matches valid tag names, anchored at the start and
/// end of the matched string.
static ANCHORED_TAG_REGEXP: Lazy<Regex> = Lazy::new(|| Regex::new(&anchored!(TAG)).unwrap());

/// ANCHORED_DIGEST_REGEXP matches valid digests, anchored at the start and
/// end of the matched string.
pub(crate) static ANCHORED_DIGEST_REGEXP: Lazy<Regex> =
    Lazy::new(|| Regex::new(&anchored!(DIGEST_PAT)).unwrap());

/// alphanumeric defines the alphanumeric atom, typically a
/// component of names. This only allows lower case characters and digits.
const ALPHANUMERIC: &str = "[a-z0-9]+";

/// separator defines the separators allowed to be embedded in name
/// components. This allows one period, one or two underscore and multiple
/// dashes. Repeated dashes and underscores are intentionally treated
/// differently. In order to support valid hostnames as name components,
/// supporting repeated dash was added. Additionally double underscore is
/// now allowed as a separator to loosen the restriction for previously
/// supported names.
const SEPARATOR: &str = "(?:[._]|__|[-]+)";

/// domainNameComponent restricts the registry domain component of a
/// repository name to start with a component as defined by DomainRegexp.
const DOMAIN_NAME_COMPONENT: &str = "(?:[a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])";

/// optionalPort matches an optional port-number including the port separator
/// (e.g. ":80").
const OPTIONAL_PORT: &str = r#"(?::[0-9]+)?"#;

/// tagPat matches valid tag names. From docker/docker:graph/tags.go.
const TAG: &str = r"[\w][\w.-]{0,127}";

/// digestPat matches well-formed digests, including algorithm (e.g. "sha256:<encoded>").
const DIGEST_PAT: &str = r"[A-Za-z][A-Za-z0-9]*(?:[-_+.][A-Za-z][A-Za-z0-9]*)*[:][[:xdigit:]]{32,}";

/// identifier is the format for a content addressable identifier using sha256.
/// These identifiers are like digests without the algorithm, since sha256 is used.
const IDENTIFIER: &str = r"([a-f0-9]{64})";

/// ipv6address are enclosed between square brackets and may be represented
/// in many ways, see rfc5952. Only IPv6 in compressed or uncompressed format
/// are allowed, IPv6 zone identifiers (rfc6874) or Special addresses such as
/// IPv4-Mapped are deliberately excluded.
const IPV6_ADDRESS: &str = r"\[(?:[a-fA-F0-9:]+)\]";

// Helper functions

// functions to generate the regex patterns

/// domainName defines the structure of potential domain components
/// that may be part of image names. This is purposely a subset of what is
/// allowed by DNS to ensure backwards compatibility with Docker image
/// names. This includes IPv4 addresses on decimal format.
fn domain_name() -> String {
    format!(
        "{DOMAIN_NAME_COMPONENT}{}",
        any_times(&format!("\\.{DOMAIN_NAME_COMPONENT}"))
    )
}

/// host defines the structure of potential domains based on the URI
/// Host subcomponent on rfc3986. It may be a subset of DNS domain name,
/// or an IPv4 address in decimal format, or an IPv6 address between square
/// brackets (excluding zone identifiers as defined by rfc6874 or special
/// addresses such as IPv4-Mapped).
fn host() -> String {
    let domain_name = domain_name();
    format!("(?:{domain_name}|{IPV6_ADDRESS})",)
}

/// allowed by the URI Host subcomponent on rfc3986 to ensure backwards
/// compatibility with Docker image names.
fn domain_and_port() -> String {
    let host = host();
    format!("{host}{OPTIONAL_PORT}")
}

/// `path_component` restricts path-components to start with an alphanumeric
/// character, with following parts able to be separated by a separator
/// (one period, one or two underscore and multiple dashes).
fn path_component() -> String {
    format!(
        "{ALPHANUMERIC}{}",
        any_times(&format!("{SEPARATOR}{ALPHANUMERIC}"))
    )
}

/// remote_name matches the remote-name of a repository. It consists of one
/// or more forward slash (/) delimited path-components:
///
/// pathComponent[[/pathComponent] ...] // e.g., "library/ubuntu"
fn remote_name() -> String {
    let path_component = path_component();
    format!(
        "{path_component}{}",
        any_times(&format!("/{path_component}"))
    )
}

fn name_pat() -> String {
    let opt_domain_and_port = optional!(&domain_and_port(), "/");
    format!("{opt_domain_and_port}{}", remote_name())
}

fn reference_pat() -> String {
    anchored!(
        &capture(&name_pat()),
        &optional!(":", &capture(TAG)),
        &optional!("@", &capture(DIGEST_PAT))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_regexp() {
        let complete_domain_regex = Regex::new(&format!("^{}$", DOMAIN_REGEX.as_str())).unwrap();

        macro_rules! test_domain {
            ($test:expr, $match:expr) => {
                assert_eq!(complete_domain_regex.is_match($test), $match,);
            };
        }

        test_domain!("test.com", true);
        test_domain!("test.com:10304", true);
        test_domain!("test.com:http", false);
        test_domain!("localhost", true);
        test_domain!("localhost:8080", true);
        test_domain!("a", true);
        test_domain!("a.b", true);
        test_domain!("ab.cd.com", true);
        test_domain!("a-b.com", true);
        test_domain!("-ab.com", false);
        test_domain!("ab-.com", false);
        test_domain!("ab.c-om", true);
        test_domain!("ab.-com", false);
        test_domain!("ab.com-", false);
        test_domain!("0101.com", true);
        test_domain!("001a.com", true);
        test_domain!("b.gbc.io:443", true);
        test_domain!("b.gbc.io", true);
        // ☃.com in punycode
        test_domain!("xn--n3h.com", true);
        // uppercase character
        test_domain!("Asdf.com", true);
        // ipv4
        test_domain!("192.168.1.1:75050", true);
        // port with more than 5 digits, it will fail on validation
        test_domain!("192.168.1.1:750050", true);
        // ipv6 compressed
        test_domain!("[fd00:1:2::3]:75050", true);
        // ipv6 wrong port separator
        test_domain!("[fd00:1:2::3]75050", false);
        // ipv6 wrong port separator
        test_domain!("[fd00:1:2::3]::75050", false);
        // ipv6 with zone
        test_domain!("[fd00:1:2::3%eth0]:75050", false);
        // ipv6 wrong format, will fail in validation
        test_domain!("[fd00123123123]:75050", true);
        // ipv6 long format
        test_domain!("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:75050", true);
        // ipv6 long format and invalid port, it will fail in validation
        test_domain!("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:750505", true);
        // bad ipv6 without square brackets
        test_domain!("fd00:1:2::3:75050", false);
    }

    #[test]
    fn test_full_name_regex() {
        // 3 captures: full match, domain, remote name
        assert_eq!(ANCHORED_NAME_REGEXP.captures_len(), 3);

        println!("ANCHORED_NAME_REGEXP: {}", ANCHORED_NAME_REGEXP.as_str());
        println!("ANCHORED_NAME_REGEXP: {:?}", ANCHORED_NAME_REGEXP);

        macro_rules! test_name {
            ($text:expr, $captures:expr) => {
                let text = $text;
                let captures = ANCHORED_NAME_REGEXP.captures(&text);

                let captures = captures.unwrap();
                assert_eq!(captures.len(), $captures.len() + 1);
                for (i, capture) in $captures.into_iter().enumerate() {
                    assert_eq!(captures.get(i + 1).map(|s| s.as_str()), capture);
                }
            };
            ($text:expr) => {
                assert!(!ANCHORED_NAME_REGEXP.is_match($text));
            };
        }

        test_name!("");
        test_name!("short", vec![None, Some("short")]);
        test_name!("simple/name", vec![Some("simple"), Some("name")]);
        test_name!("library/ubuntu", vec![Some("library"), Some("ubuntu")]);
        test_name!(
            "docker/stevvooe/app",
            vec![Some("docker"), Some("stevvooe/app")]
        );
        test_name!(
            "aa/aa/aa/aa/aa/aa/aa/aa/aa/bb/bb/bb/bb/bb/bb",
            vec![
                Some("aa"),
                Some("aa/aa/aa/aa/aa/aa/aa/aa/bb/bb/bb/bb/bb/bb")
            ]
        );
        test_name!("aa/aa/bb/bb/bb", vec![Some("aa"), Some("aa/bb/bb/bb")]);
        test_name!("a/a/a/a", vec![Some("a"), Some("a/a/a")]);
        test_name!("a/a/a/a/");
        test_name!("a//a/a");
        test_name!("a", vec![None, Some("a")]);
        test_name!("a/aa", vec![Some("a"), Some("aa")]);
        test_name!("a/aa/a", vec![Some("a"), Some("aa/a")]);
        test_name!("foo.com", vec![None, Some("foo.com")]);
        test_name!("foo.com/");
        test_name!("foo.com:8080/bar", vec![Some("foo.com:8080"), Some("bar")]);
        test_name!("foo.com:8080/bar", vec![Some("foo.com:8080"), Some("bar")]);
        test_name!("foo.com/bar", vec![Some("foo.com"), Some("bar")]);
        test_name!("foo.com/bar/baz", vec![Some("foo.com"), Some("bar/baz")]);
        test_name!(
            "localhost:8080/bar",
            vec![Some("localhost:8080"), Some("bar")]
        );
        test_name!(
            "sub-dom1.foo.com/bar/baz/quux",
            vec![Some("sub-dom1.foo.com"), Some("bar/baz/quux")]
        );
        test_name!(
            "blog.foo.com/bar/baz",
            vec![Some("blog.foo.com"), Some("bar/baz")]
        );
        test_name!("a^a");
        test_name!("aa/asdf$$^/aa");
        test_name!("asdf$$^/aa");
        test_name!("aa-a/a", vec![Some("aa-a"), Some("a")]);
        test_name!(
            format!("{}a", "a/".repeat(128)),
            vec![Some("a"), Some(&format!("{}a", "a/".repeat(127)))]
        );
        test_name!("a-/a/a/a");
        test_name!("foo.com/a-/a/a");
        test_name!("-foo/bar");
        test_name!("foo/bar-");
        test_name!("foo-/bar");
        test_name!("foo/-bar");
        test_name!("_foo/bar");
        test_name!("foo_bar", vec![None, Some("foo_bar")]);
        test_name!("foo_bar.com", vec![None, Some("foo_bar.com")]);
        test_name!("foo_bar.com:8080");
        test_name!("foo_bar.com:8080/app");
        test_name!("foo.com/foo_bar", vec![Some("foo.com"), Some("foo_bar")]);
        test_name!("____/____");
        test_name!("_docker/_docker");
        test_name!("docker_/docker_");
        test_name!(
            "b.gcr.io/test.example.com/my-app",
            vec![Some("b.gcr.io"), Some("test.example.com/my-app")]
        );
        test_name!(
            // ☃.com in punycode
            "xn--n3h.com/myimage",
            vec![Some("xn--n3h.com"), Some("myimage")]
        );
        test_name!(
            // 🐳.com in punycode
            "xn--7o8h.com/myimage",
            vec![Some("xn--7o8h.com"), Some("myimage")]
        );
        test_name!(
            // 🐳.com in punycode
            "example.com/xn--7o8h.com/myimage",
            vec![Some("example.com"), Some("xn--7o8h.com/myimage")]
        );
        test_name!(
            "example.com/some_separator__underscore/myimage",
            vec![
                Some("example.com"),
                Some("some_separator__underscore/myimage")
            ]
        );
        test_name!("example.com/__underscore/myimage");
        test_name!("example.com/..dots/myimage");
        test_name!("example.com/.dots/myimage");
        test_name!("example.com/nodouble..dots/myimage");
        test_name!("docker./docker");
        test_name!(".docker/docker");
        test_name!("docker-/docker");
        test_name!("-docker/docker");
        test_name!("do..cker/docker");
        test_name!("do__cker:8080/docker");
        test_name!("do__cker/docker", vec![None, Some("do__cker/docker")]);
        test_name!(
            "b.gcr.io/test.example.com/my-app",
            vec![Some("b.gcr.io"), Some("test.example.com/my-app")]
        );
        test_name!(
            "registry.io/foo/project--id.module--name.ver---sion--name",
            vec![
                Some("registry.io"),
                Some("foo/project--id.module--name.ver---sion--name")
            ]
        );
        test_name!("Asdf.com/foo/bar", vec![Some("Asdf.com"), Some("foo/bar")]);
        test_name!("Foo/FarB");
    }

    #[test]
    fn test_reference_regexp() {
        assert_eq!(REFERENCE_REGEX.captures_len(), 4);

        macro_rules! test_reference {
            ($text:expr, $captures:expr) => {
                let text = $text;
                let captures = REFERENCE_REGEX.captures(&text).unwrap();
                assert_eq!(captures.len(), $captures.len() + 1);
                for (i, capture) in $captures.into_iter().enumerate() {
                    assert_eq!(captures.get(i + 1).map(|s| s.as_str()), capture);
                }
            };
            ($text:expr) => {
                assert!(!REFERENCE_REGEX.is_match($text));
            };
        }

        test_reference!(
            "registry.com:8080/myapp",
            vec![Some("registry.com:8080/myapp"), None, None]
        );

        test_reference!(
            "registry.com:8080/myapp:tag",
            vec![Some("registry.com:8080/myapp"), Some("tag"), None]
        );

        test_reference!(
            "registry.com:8080/myapp@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912",
            vec![Some("registry.com:8080/myapp"), None, Some("sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912")]
        );
        test_reference!("registry.com:8080/myapp@sha256:badbadbadbad");
        test_reference!("registry.com:8080/myapp:invalid~tag");
        test_reference!("bad_hostname.com:8080/myapp:tag");
        test_reference!(
            "localhost:8080@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912", 
            vec![Some("localhost"), Some("8080"), Some("sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912")]
        );
        test_reference!("localhost:8080/name@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912",
            vec![Some("localhost:8080/name"), None, Some("sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912")]
        );
        test_reference!(
            "localhost@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912",
            vec![
                Some("localhost"),
                None,
                Some("sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912")
            ]
        );
        test_reference!(
            "localhost:http/name@sha256:be178c0543eb17f5f3043021c9e5fcf30285e557a4fc309cce97ff9ca6182912"
        );
        test_reference!("registry.com:8080/myapp@bad");
        test_reference!("registry.com:8080/myapp@2bad");
    }

    #[test]
    fn test_identifier_regexp() {
        macro_rules! test_identifier {
            ($test:expr, $match:expr) => {
                assert_eq!(ANCHORED_IDENTIFIER_REGEXP.is_match($test), $match,);
            };
        }

        test_identifier!(
            "da304e823d8ca2b9d863a3c897baeb852ba21ea9a9f1414736394ae7fcaf9821",
            true
        );
        test_identifier!(
            "7EC43B381E5AEFE6E04EFB0B3F0693FF2A4A50652D64AEC573905F2DB5889A1C",
            false
        );
        test_identifier!(
            "da304e823d8ca2b9d863a3c897baeb852ba21ea9a9f1414736394ae7fcaf",
            false
        );
        test_identifier!(
            "sha256:da304e823d8ca2b9d863a3c897baeb852ba21ea9a9f1414736394ae7fcaf9821",
            false
        );
        test_identifier!(
            "da304e823d8ca2b9d863a3c897baeb852ba21ea9a9f1414736394ae7fcaf98218482",
            false
        );
    }
}
