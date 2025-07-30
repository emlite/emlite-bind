use crate::any::Any;
use crate::record::Record;

/// ECMAScript ordinary object backed by an `emlite::Val`.
pub type Object = Record<Any, Any>;
