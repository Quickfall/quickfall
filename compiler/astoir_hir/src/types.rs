//! Additional information for scope stored types such as function impls

use typing::raw::RawType;

use crate::func::HIRFunction;

pub struct ScopeStoredType {
    pub t: RawType,
    pub function_implementations: Vec<&'static HIRFunction>,
}
