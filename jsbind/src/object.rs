use crate::Any;
use crate::utils::bind;

/// ECMAScript ordinary object backed by an `emlite::Val`.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Object {
    /// Underlying handle (guaranteed to be an object at runtime).
    inner: emlite::Val,
}

impl Object {
    /// Create a brand‑new empty object (same as `{}` in JavaScript).
    pub fn new() -> Object {
        Self {
            inner: emlite::Val::object(),
        }
    }

    /// `=== null` – not just falsy!
    #[inline]
    pub fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    /// `=== undefined` – distinguishes from `null`.
    #[inline]
    pub fn is_undefined(&self) -> bool {
        self.inner.is_undefined()
    }

    /// `Object.prototype.hasOwnProperty.call(obj, prop)`
    ///
    /// Returns `true` if the property exists directly on the object
    /// (enumerable or not).  Inherited properties through the prototype chain
    /// return `false`.
    pub fn has_own_property(&self, prop: &str) -> bool {
        self.inner.has_own_property(prop)
    }

    pub fn set(&self, item: &str, val: &Any) {
        self.inner.set(item, val.clone());
    }

    /// Returns whether a value exists in the sequence.
    pub fn has<T>(&self, item: T) -> bool
    where
        emlite::Val: From<T>,
    {
        self.inner.has(item)
    }

    pub fn get(&self, item: &Any) -> Any {
        self.inner.get(item.clone()).as_::<Any>()
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

bind!(Object);
