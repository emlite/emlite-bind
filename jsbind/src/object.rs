use crate::any::Any;
use crate::utils::*;

/// ECMAScript ordinary object backed by an `emlite::Val`.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Object {
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

    /// Gets the Object constructor function
    /// @returns emlite::Val representing the Object constructor
    pub fn instance() -> emlite::Val {
        emlite::Val::global("Object")
    }

    /// Creates an object with the specified prototype object and properties
    /// @param prototype the object to use as the prototype, or null for no prototype
    /// @returns a new object with the specified prototype
    pub fn create(prototype: &emlite::Val) -> emlite::Val {
        emlite::Val::global("Object").call("create", &[prototype.clone()])
    }

    /// Creates an object with the specified prototype object and properties
    /// @param prototype the object to use as the prototype, or null for no prototype
    /// @param properties an object whose enumerable own properties specify property descriptors
    /// @returns a new object with the specified prototype and properties
    pub fn create_with_properties(prototype: &emlite::Val, properties: &emlite::Val) -> emlite::Val {
        emlite::Val::global("Object").call("create", &[prototype.clone(), properties.clone()])
    }

    /// Sets the prototype (i.e., the internal [[Prototype]] property) of a specified object
    /// @param obj the object which is to have its prototype set
    /// @param prototype the object's new prototype (an object or null)
    /// @returns the specified object
    pub fn set_prototype_of(obj: &emlite::Val, prototype: &emlite::Val) -> emlite::Val {
        emlite::Val::global("Object").call("setPrototypeOf", &[obj.clone(), prototype.clone()])
    }
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

bind!(Object);
impl_dyn_cast!(Object);
