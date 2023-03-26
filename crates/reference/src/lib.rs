//! A parser for image references.
//!
//! Based on the canonical [Docker image reference parser](https://github.com/distribution/distribution/tree/main/reference)
//!
//! ## Example
//!
//! ```
//! use buildkit_rs_reference::Reference;
//!
//! // Parse a reference with no domain, an incomplete path, and a tag
//! let image = "alpine:latest";
//!
//! let reference = Reference::parse_normalized_named(image).unwrap();
//! assert_eq!(reference.domain(), "docker.io");
//! assert_eq!(reference.path().as_deref(), Some("library/alpine"));
//! assert_eq!(reference.tag(), Some("latest"));
//!
//! // Parse a reference with a domain and digest
//! let image = "b.gcr.io/alpine@sha256:86e0e091d0da6bde2456dbb48306f3956bbeb2eae1b5b9a43045843f69fe4aaa";
//!
//! let reference = Reference::parse_normalized_named(image).unwrap();
//! assert_eq!(reference.domain(), "b.gcr.io");
//! assert_eq!(reference.path().as_deref(), Some("alpine"));
//! assert_eq!(reference.digest(), Some("sha256:86e0e091d0da6bde2456dbb48306f3956bbeb2eae1b5b9a43045843f69fe4aaa"));
//! ```

pub(crate) mod consts;
pub(crate) mod error;
pub(crate) mod reference;
pub(crate) mod regex;

pub use error::Error;
pub use reference::{Reference, Repository};
