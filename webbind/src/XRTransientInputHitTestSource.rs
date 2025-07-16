use super::*;

/// The XRTransientInputHitTestSource class.
/// [`XRTransientInputHitTestSource`](https://developer.mozilla.org/en-US/docs/Web/API/XRTransientInputHitTestSource)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRTransientInputHitTestSource {
    inner: Any,
}
impl FromVal for XRTransientInputHitTestSource {
    fn from_val(v: &Any) -> Self {
        XRTransientInputHitTestSource {
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
impl core::ops::Deref for XRTransientInputHitTestSource {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRTransientInputHitTestSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRTransientInputHitTestSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRTransientInputHitTestSource {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRTransientInputHitTestSource> for Any {
    fn from(s: XRTransientInputHitTestSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRTransientInputHitTestSource> for Any {
    fn from(s: &XRTransientInputHitTestSource) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRTransientInputHitTestSource);

impl XRTransientInputHitTestSource {
    /// The cancel method.
    /// [`XRTransientInputHitTestSource.cancel`](https://developer.mozilla.org/en-US/docs/Web/API/XRTransientInputHitTestSource/cancel)
    pub fn cancel(&self) -> Undefined {
        self.inner.call("cancel", &[]).as_::<Undefined>()
    }
}
