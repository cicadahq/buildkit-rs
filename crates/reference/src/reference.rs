use crate::{
    consts::{DEFAULT_DOMAIN, LEGACY_DEFAULT_DOMAIN, NAME_TOTAL_LENGTH_MAX, OFFICIAL_REPO_PREFIX},
    regex::{ANCHORED_IDENTIFIER_REGEXP, ANCHORED_NAME_REGEXP, REFERENCE_REGEX},
    Error,
};
use std::{borrow::Cow, cmp::Ordering, fmt};

/// A repository is the part of a reference before the tag or digest
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository {
    /// The domain of the repository (e.g. `docker.io`)
    pub domain: Option<String>,
    /// The path of the repository (e.g. `library/alpine`)
    pub path: Option<String>,
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.domain_or_default())?;

        if let Some(path) = self.normalized_path() {
            write!(f, "/{path}")?;
        }

        Ok(())
    }
}

impl Repository {
    /// Returns the domain of the repository, or the default domain (`docker.io`) if it is not set
    pub fn domain_or_default(&self) -> &str {
        self.domain.as_deref().unwrap_or(DEFAULT_DOMAIN)
    }

    /// Normalizes the path of the repository if it is an official repository
    ///
    /// (i.e. `library/foo` -> `foo`)
    pub fn normalized_path(&self) -> Option<Cow<str>> {
        let path = self.path.as_deref()?;
        if matches!(
            self.domain.as_deref(),
            None | Some(DEFAULT_DOMAIN) | Some(LEGACY_DEFAULT_DOMAIN)
        ) && !path.contains('/')
        {
            Some(format!("{OFFICIAL_REPO_PREFIX}{}", path).into())
        } else {
            Some(path.into())
        }
    }
}

/// A reference is a named reference to an image
///
/// Examples:
/// - `docker.io/library/alpine`
/// - `docker.io/library/alpine:latest`
/// - `docker.io/library/alpine@sha256:86e0e091d0da6bde2456dbb48306f3956bbeb2eae1b5b9a43045843f69fe4aaa`
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// The repository of the reference, (e.g. `docker.io/library/alpine`)
    pub repository: Repository,
    /// The tag of the reference, (e.g. `latest`)
    pub tag: Option<String>,
    /// The digest of the reference, (e.g. `sha256:86e0e091d0da6bde2456dbb48306f3956bbeb2eae1b5b9a43045843f69fe4aaa`)
    pub digest: Option<String>,
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repository)?;

        if let Some(tag) = &self.tag {
            write!(f, ":{tag}")?;
        }

        if let Some(digest) = &self.digest {
            write!(f, "@{digest}")?;
        }

        Ok(())
    }
}

impl Reference {
    /// Parse parses `s` and returns a syntactically valid Reference
    pub fn parse(s: impl AsRef<str>) -> Result<Self, Error> {
        let s = s.as_ref();

        // TODO: This maybe should use a real parser instead of regexes, the regex is REALLY BIG
        // and makes a 60Mb allocationthis was copied from the original go code, but we can do better :)
        let matches = REFERENCE_REGEX.captures(s);
        let Some(matches) = matches else {
            if s.is_empty() {
                return Err(Error::NameEmpty);
            }
            if REFERENCE_REGEX.captures(&s.to_lowercase()).is_some() {
                return Err(Error::NameContainsUppercase)
            }
            return Err(Error::InvalidReferenceFormat)
        };

        if matches.get(0).unwrap().as_str().len() > NAME_TOTAL_LENGTH_MAX {
            return Err(Error::NameTooLong);
        }

        let name_match = ANCHORED_NAME_REGEXP.captures(
            matches
                .get(1)
                .ok_or(Error::InvalidReferenceFormat)?
                .as_str(),
        );
        let Some(name_match) = name_match else {
            return Err(Error::InvalidReferenceFormat)
        };

        let repo = match name_match.get(1) {
            Some(domain) => Repository {
                domain: Some(domain.as_str().into()),
                path: name_match.get(2).map(|m| m.as_str().into()),
            },
            None => Repository {
                domain: None,
                path: name_match.get(2).map(|m| m.as_str().into()),
            },
        };

        let tag = matches.get(2).map(|m| m.as_str().to_owned());

        // TODO: Actually validate the digest
        let digest = matches.get(3).map(|m| m.as_str().to_owned());

        Ok(Reference {
            repository: repo,
            tag,
            digest,
        })
    }

    /// Parses a string into a named reference transforming a familiar name from Docker UI
    /// to a fully qualified reference
    pub fn parse_normalized_named(s: &str) -> Result<Self, Error> {
        if ANCHORED_IDENTIFIER_REGEXP.is_match(s) {
            return Err(Error::NameIdentifier);
        }

        let (domain, remainder) = split_docker_domain(s);
        let remote = remainder
            .find(':')
            .map(|i| &remainder[..i])
            .unwrap_or(&remainder);

        if remote.contains(|c: char| c.is_uppercase()) {
            return Err(Error::NameContainsUppercase);
        }

        Self::parse(format!("{domain}/{remainder}"))
    }

    /// Returns the domain of the reference, or the default domain (`docker.io`) if it is not set
    pub fn domain(&self) -> &str {
        self.repository.domain_or_default()
    }

    /// Returns the path of the reference, normalized if it is an official repository
    pub fn path(&self) -> Option<Cow<str>> {
        self.repository.normalized_path()
    }

    /// The tag of the reference, or `None` if it is not set
    pub fn tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }

    /// The digest of the reference, or `None` if it is not set
    pub fn digest(&self) -> Option<&str> {
        self.digest.as_deref()
    }

    /// `rank_ord` returns a [Ordering] based on the following rules preferring higher
    /// information references, then by the lexicographical ordering of the reference string:
    ///
    /// | Rule | Example |
    /// |------|---------|
    /// | \[Named\] + \[Tagged\] + \[Digested\] | `docker.io/library/busybox:latest@sha256:<digest>` |
    /// | \[Named\] + \[Tagged\]                | `docker.io/library/busybox:latest` |
    /// | \[Named\] + \[Digested\]              | `docker.io/library/busybo@sha256:<digest>` |
    /// | \[Named\]                             | `docker.io/library/busybox` |
    /// | \[Digested\]                          | `docker.io@sha256:<digest>` |
    /// | Error                                 | The reference is not valid due to not matching any of the above rules |
    ///
    /// [Original](https://github.com/distribution/distribution/blob/e5d5810851d1f17a5070e9b6f940d8af98ea3c29/reference/sort.go)
    pub fn rank_ord(&self, other: &Self) -> Ordering {
        let get_order = |r: &Reference| {
            if r.repository.path.is_some() {
                if r.tag.is_some() {
                    if r.digest.is_some() {
                        1
                    } else {
                        2
                    }
                } else if r.digest.is_some() {
                    3
                } else {
                    4
                }
            } else if r.digest.is_some() {
                5
            } else {
                6
            }
        };

        let order = get_order(self);
        let other_order = get_order(other);

        if order == other_order {
            // Convert to a string and compare
            self.to_string().cmp(&other.to_string())
        } else {
            order.cmp(&other_order)
        }
    }
}

/// splitDockerDomain splits a repository name to domain and remote-name.
/// If no valid domain is found, the default domain is used. Repository name
/// needs to be already validated before.
///
/// https://github.com/distribution/distribution/blob/e5d5810851d1f17a5070e9b6f940d8af98ea3c29/reference/normalize.go#L126
fn split_docker_domain<'a>(name: &'a str) -> (&'a str, Cow<'a, str>) {
    let mut domain: &str;
    let mut remainder: Cow<'a, str>;

    if let Some(i) = name.find('/') {
        if !name[..i].chars().any(|c| c == '.' || c == ':')
            && &name[..i] != "localhost"
            && name[..i].to_lowercase() == name[..i]
        {
            domain = DEFAULT_DOMAIN;
            remainder = name.into();
        } else {
            domain = &name[..i];
            remainder = (&name[i + 1..]).into();
        }
    } else {
        domain = DEFAULT_DOMAIN;
        remainder = name.into();
    }

    if domain == LEGACY_DEFAULT_DOMAIN {
        domain = DEFAULT_DOMAIN;
    }

    if domain == DEFAULT_DOMAIN && !remainder.contains('/') {
        remainder = format!("{OFFICIAL_REPO_PREFIX}{remainder}").into();
    }

    (domain, remainder)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_normalized_named_valid_repo_names() {
        let valid_repos = [
            "docker/docker",
            "library/debian",
            "debian",
            "docker.io/docker/docker",
            "docker.io/library/debian",
            "docker.io/debian",
            "index.docker.io/docker/docker",
            "index.docker.io/library/debian",
            "index.docker.io/debian",
            "127.0.0.1:5000/docker/docker",
            "127.0.0.1:5000/library/debian",
            "127.0.0.1:5000/debian",
            "192.168.0.1",
            "192.168.0.1:80",
            "192.168.0.1:8/debian",
            "192.168.0.2:25000/debian",
            "thisisthesongthatneverendsitgoesonandonandonthisisthesongthatnev",
            "[fc00::1]:5000/docker",
            "[fc00::1]:5000/docker/docker",
            "[fc00:1:2:3:4:5:6:7]:5000/library/debian",
            // This test case was moved from invalid to valid since it is valid input
            // when specified with a hostname, it removes the ambiguity from about
            // whether the value is an identifier or repository name
            "docker.io/1a3f5e7d9c1b3a5f7e9d1c3b5a7f9e1d3c5b7a9f1e3d5d7c9b1a3f5e7d9c1b3a",
            "Docker/docker",
            "DOCKER/docker",
        ];

        for repo in valid_repos {
            assert!(Reference::parse_normalized_named(repo).is_ok());
        }
    }

    #[test]
    fn test_normalized_named_invalid_repo_names() {
        let invalid_repos = [
            "https://github.com/docker/docker",
            "docker/Docker",
            "-docker",
            "-docker/docker",
            "-docker.io/docker/docker",
            "docker///docker",
            "docker.io/docker/Docker",
            "docker.io/docker///docker",
            "[fc00::1]",
            "[fc00::1]:5000",
            "fc00::1:5000/debian",
            "[fe80::1%eth0]:5000/debian",
            "[2001:db8:3:4::192.0.2.33]:5000/debian",
            "1a3f5e7d9c1b3a5f7e9d1c3b5a7f9e1d3c5b7a9f1e3d5d7c9b1a3f5e7d9c1b3a",
        ];

        for repo in invalid_repos {
            assert!(Reference::parse_normalized_named(repo).is_err());
        }
    }

    #[test]
    fn test_normalized_named_valid_remote_name() {
        let valid_remote_names = [
            // Sanity check.
            "docker/docker",
            // Allow 64-character non-hexadecimal names (hexadecimal names are forbidden).
            "thisisthesongthatneverendsitgoesonandonandonthisisthesongthatnev",
            // Allow embedded hyphens.
            "docker-rules/docker",
            // Allow multiple hyphens as well.
            "docker---rules/docker",
            // Username doc and image name docker being tested.
            "doc/docker",
            // single character names are now allowed.
            "d/docker",
            "jess/t",
            // Consecutive underscores.
            "dock__er/docker",
        ];

        for remote_name in valid_remote_names {
            assert!(Reference::parse_normalized_named(remote_name).is_ok());
        }
    }

    #[test]
    fn test_normalized_named_invalid_remote_name() {
        let invalid_remote_names = [
            // Disallow capital letters.
            "docker/Docker",
            // Only allow one slash.
            "docker///docker",
            // Disallow 64-character hexadecimal.
            "1a3f5e7d9c1b3a5f7e9d1c3b5a7f9e1d3c5b7a9f1e3d5d7c9b1a3f5e7d9c1b3a",
            // Disallow leading and trailing hyphens in namespace.
            "-docker/docker",
            "docker-/docker",
            "-docker-/docker",
            // Don't allow underscores everywhere (as opposed to hyphens).
            "____/____",
            "_docker/_docker",
            // Disallow consecutive periods.
            "dock..er/docker",
            "dock_.er/docker",
            "dock-.er/docker",
            // No repository.
            "docker/",
            // namespace too long
            "this_is_not_a_valid_namespace_because_its_lenth_is_greater_than_255_this_is_not_a_valid_namespace_because_its_lenth_is_greater_than_255_this_is_not_a_valid_namespace_because_its_lenth_is_greater_than_255_this_is_not_a_valid_namespace_because_its_lenth_is_greater_than_255/docker",
        ];

        for remote_name in invalid_remote_names {
            assert!(Reference::parse_normalized_named(remote_name).is_err());
        }
    }

    #[test]
    fn test_parse_reference_with_tag_and_digest() {
        let short_ref = "busybox:latest@sha256:86e0e091d0da6bde2456dbb48306f3956bbeb2eae1b5b9a43045843f69fe4aaa";
        let normalized = Reference::parse_normalized_named(short_ref).unwrap();

        assert_eq!(
            normalized,
            Reference {
                repository: Repository {
                    domain: Some("docker.io".into()),
                    path: Some("library/busybox".into())
                },
                tag: Some("latest".into()),
                digest: Some(
                    "sha256:86e0e091d0da6bde2456dbb48306f3956bbeb2eae1b5b9a43045843f69fe4aaa"
                        .into()
                )
            }
        );

        assert_eq!(
            normalized.to_string(),
            "docker.io/library/busybox:latest@sha256:86e0e091d0da6bde2456dbb48306f3956bbeb2eae1b5b9a43045843f69fe4aaa"
        );
    }
}
