mod azync;
mod definition;
mod diff;
mod exec;
mod marshal;
mod sourcemap;
mod state;
mod source;
mod meta;
mod serialize;

type Digest = String;

#[derive(Debug, thiserror::Error)]
pub enum Error {}
