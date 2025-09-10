use super::*;

/// The SharedStorageModifierMethodOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageModifierMethodOptions {
    inner: Any,
}

impl FromVal for SharedStorageModifierMethodOptions {
    fn from_val(v: &Any) -> Self {
        SharedStorageModifierMethodOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SharedStorageModifierMethodOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SharedStorageModifierMethodOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SharedStorageModifierMethodOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SharedStorageModifierMethodOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SharedStorageModifierMethodOptions> for Any {
    fn from(s: SharedStorageModifierMethodOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SharedStorageModifierMethodOptions> for Any {
    fn from(s: &SharedStorageModifierMethodOptions) -> Any {
        s.inner.clone()
    }
}

impl SharedStorageModifierMethodOptions {
    /// Getter of the `withLock` attribute.
    pub fn with_lock(&self) -> JsString {
        self.inner.get("withLock").as_::<JsString>()
    }

    /// Setter of the `withLock` attribute.
    pub fn set_with_lock(&mut self, value: &JsString) {
        self.inner.set("withLock", value);
    }
}
