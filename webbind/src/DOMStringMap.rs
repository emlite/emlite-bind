use super::*;

/// The DOMStringMap class.
/// [`DOMStringMap`](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMStringMap {
    inner: Any,
}
impl FromVal for DOMStringMap {
    fn from_val(v: &Any) -> Self {
        DOMStringMap {
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
impl core::ops::Deref for DOMStringMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMStringMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DOMStringMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DOMStringMap {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DOMStringMap> for Any {
    fn from(s: DOMStringMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DOMStringMap> for Any {
    fn from(s: &DOMStringMap) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DOMStringMap);
