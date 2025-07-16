use super::*;

/// The SharedStorageDeleteMethod class.
/// [`SharedStorageDeleteMethod`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageDeleteMethod)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageDeleteMethod {
    inner: SharedStorageModifierMethod,
}
impl FromVal for SharedStorageDeleteMethod {
    fn from_val(v: &Any) -> Self {
        SharedStorageDeleteMethod {
            inner: SharedStorageModifierMethod::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SharedStorageDeleteMethod {
    type Target = SharedStorageModifierMethod;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageDeleteMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageDeleteMethod {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageDeleteMethod {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageDeleteMethod> for Any {
    fn from(s: SharedStorageDeleteMethod) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageDeleteMethod> for Any {
    fn from(s: &SharedStorageDeleteMethod) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedStorageDeleteMethod);

impl SharedStorageDeleteMethod {
    /// The `new SharedStorageDeleteMethod(..)` constructor, creating a new SharedStorageDeleteMethod instance
    pub fn new0(key: &str) -> SharedStorageDeleteMethod {
        Self {
            inner: Any::global("SharedStorageDeleteMethod")
                .new(&[key.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }

    /// The `new SharedStorageDeleteMethod(..)` constructor, creating a new SharedStorageDeleteMethod instance
    pub fn new1(
        key: &str,
        options: &SharedStorageModifierMethodOptions,
    ) -> SharedStorageDeleteMethod {
        Self {
            inner: Any::global("SharedStorageDeleteMethod")
                .new(&[key.into(), options.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }
}
