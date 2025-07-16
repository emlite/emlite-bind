use super::*;

/// The SharedStorageAppendMethod class.
/// [`SharedStorageAppendMethod`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageAppendMethod)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageAppendMethod {
    inner: SharedStorageModifierMethod,
}
impl FromVal for SharedStorageAppendMethod {
    fn from_val(v: &Any) -> Self {
        SharedStorageAppendMethod {
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
impl core::ops::Deref for SharedStorageAppendMethod {
    type Target = SharedStorageModifierMethod;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageAppendMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageAppendMethod {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageAppendMethod {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageAppendMethod> for Any {
    fn from(s: SharedStorageAppendMethod) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageAppendMethod> for Any {
    fn from(s: &SharedStorageAppendMethod) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedStorageAppendMethod);

impl SharedStorageAppendMethod {
    /// The `new SharedStorageAppendMethod(..)` constructor, creating a new SharedStorageAppendMethod instance
    pub fn new0(key: &str, value: &str) -> SharedStorageAppendMethod {
        Self {
            inner: Any::global("SharedStorageAppendMethod")
                .new(&[key.into(), value.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }

    /// The `new SharedStorageAppendMethod(..)` constructor, creating a new SharedStorageAppendMethod instance
    pub fn new1(
        key: &str,
        value: &str,
        options: &SharedStorageModifierMethodOptions,
    ) -> SharedStorageAppendMethod {
        Self {
            inner: Any::global("SharedStorageAppendMethod")
                .new(&[key.into(), value.into(), options.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }
}
