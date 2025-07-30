use super::*;

/// The SharedStorageSetMethod class.
/// [`SharedStorageSetMethod`](https://developer.mozilla.org/en-US/docs/Web/API/SharedStorageSetMethod)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedStorageSetMethod {
    inner: SharedStorageModifierMethod,
}
impl FromVal for SharedStorageSetMethod {
    fn from_val(v: &Any) -> Self {
        SharedStorageSetMethod {
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
impl core::ops::Deref for SharedStorageSetMethod {
    type Target = SharedStorageModifierMethod;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedStorageSetMethod {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SharedStorageSetMethod {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SharedStorageSetMethod {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SharedStorageSetMethod> for Any {
    fn from(s: SharedStorageSetMethod) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SharedStorageSetMethod> for Any {
    fn from(s: &SharedStorageSetMethod) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedStorageSetMethod);

impl SharedStorageSetMethod {
    /// The `new SharedStorageSetMethod(..)` constructor, creating a new SharedStorageSetMethod instance
    pub fn new0(key: &JsString, value: &JsString) -> SharedStorageSetMethod {
        Self {
            inner: Any::global("SharedStorageSetMethod")
                .new(&[key.into(), value.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }

    /// The `new SharedStorageSetMethod(..)` constructor, creating a new SharedStorageSetMethod instance
    pub fn new1(
        key: &JsString,
        value: &JsString,
        options: &SharedStorageSetMethodOptions,
    ) -> SharedStorageSetMethod {
        Self {
            inner: Any::global("SharedStorageSetMethod")
                .new(&[key.into(), value.into(), options.into()])
                .as_::<SharedStorageModifierMethod>(),
        }
    }
}
