use super::*;

/// The SharedStorageModifierMethod class.
/// [`SharedStorageModifierMethod`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageModifierMethod)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageModifierMethod {
    inner: Any,
}

impl FromVal for SharedStorageModifierMethod {
    fn from_val(v: &Any) -> Self {
        SharedStorageModifierMethod {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SharedStorageModifierMethod {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SharedStorageModifierMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SharedStorageModifierMethod {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SharedStorageModifierMethod {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SharedStorageModifierMethod> for Any {
    fn from(s: SharedStorageModifierMethod) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SharedStorageModifierMethod> for Any {
    fn from(s: &SharedStorageModifierMethod) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SharedStorageModifierMethod);
