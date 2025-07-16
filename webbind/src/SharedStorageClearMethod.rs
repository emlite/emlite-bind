use super::*;

/// The SharedStorageClearMethod class.
/// [`SharedStorageClearMethod`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageClearMethod)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageClearMethod {
    inner: SharedStorageModifierMethod,
}
impl FromVal for SharedStorageClearMethod {
    fn from_val(v: &Any) -> Self {
        SharedStorageClearMethod {
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
impl core::ops::Deref for SharedStorageClearMethod {
    type Target = SharedStorageModifierMethod;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageClearMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageClearMethod {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageClearMethod {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageClearMethod> for Any {
    fn from(s: SharedStorageClearMethod) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageClearMethod> for Any {
    fn from(s: &SharedStorageClearMethod) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedStorageClearMethod);

impl SharedStorageClearMethod {
    /// The `new SharedStorageClearMethod(..)` constructor, creating a new SharedStorageClearMethod instance
    pub fn new0() -> SharedStorageClearMethod {
        Self {
            inner: Any::global("SharedStorageClearMethod")
                .new(&[])
                .as_::<SharedStorageModifierMethod>(),
        }
    }

    /// The `new SharedStorageClearMethod(..)` constructor, creating a new SharedStorageClearMethod instance
    pub fn new1(options: &SharedStorageModifierMethodOptions) -> SharedStorageClearMethod {
        Self {
            inner: Any::global("SharedStorageClearMethod")
                .new(&[options.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }
}
