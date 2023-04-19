//! # BuildKit Rust SDK
//!
//! This is the meta crate for the BuildKit Rust SDK, all the other crates are
//! re-exported here as modules. The other crates can be used individually if you
//! want to use only a subset of the SDK.

// pub use buildkit_rs_frontend as frontend;

/// The BuildKit LLB API
pub mod llb {
    pub use buildkit_rs_llb::*;
}

/// The BuildKit protobuf definitions
pub mod proto {
    pub use buildkit_rs_proto::*;
}

/// Parser for image references
pub mod reference {
    pub use buildkit_rs_reference::*;
}

/// Ignore file parser
pub mod ignore {
    pub use buildkit_rs_ignore::*;
}

/// Other utilities
pub mod util {
    pub use buildkit_rs_util::*;
}
