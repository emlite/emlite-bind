use super::*;

/// The XRLayer class.
/// [`XRLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRLayer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLayer {
    inner: EventTarget,
}
impl FromVal for XRLayer {
    fn from_val(v: &Any) -> Self {
        XRLayer {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRLayer {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRLayer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRLayer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRLayer> for Any {
    fn from(s: XRLayer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRLayer> for Any {
    fn from(s: &XRLayer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRLayer);
