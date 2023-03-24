use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};

static ID_ACC: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub(crate) struct OperationId(u64);

impl Clone for OperationId {
    fn clone(&self) -> Self {
        OperationId::default()
    }
}

impl Default for OperationId {
    fn default() -> Self {
        Self(ID_ACC.fetch_add(1, Ordering::Relaxed))
    }
}

impl Deref for OperationId {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl OperationId {
    pub fn new() -> Self {
        Self::default()
    }
}
