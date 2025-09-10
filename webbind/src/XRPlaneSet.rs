use super::*;

/// The XRPlaneSet class.
/// [`XRPlaneSet`](https://developer.mozilla.org/en-US/docs/Web/API/XRPlaneSet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRPlaneSet {
    inner: Any,
}

impl FromVal for XRPlaneSet {
    fn from_val(v: &Any) -> Self {
        XRPlaneSet {
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

impl core::ops::Deref for XRPlaneSet {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRPlaneSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRPlaneSet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRPlaneSet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRPlaneSet> for Any {
    fn from(s: XRPlaneSet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRPlaneSet> for Any {
    fn from(s: &XRPlaneSet) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRPlaneSet);
