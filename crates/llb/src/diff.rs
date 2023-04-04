use std::sync::Arc;
use std::sync::Mutex;

use crate::marshal::MarshalCache;
use crate::state::Constraints;
use crate::state::Output;

pub struct DiffOp {
    marshal_cache: Arc<Mutex<MarshalCache>>,
    lower: Option<Box<dyn Output>>,
    upper: Option<Box<dyn Output>>,
    output: Box<dyn Output>,
    constraints: Constraints,
}
