use super::*;




/// The XRHitTestSource class.
/// [`XRHitTestSource`](https://developer.mozilla.org/en-US/docs/Web/API/XRHitTestSource)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRHitTestSource {
    inner: Any,
}

impl FromVal for XRHitTestSource {
    fn from_val(v: &Any) -> Self {
        XRHitTestSource { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRHitTestSource {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRHitTestSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRHitTestSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRHitTestSource {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRHitTestSource> for Any {
    fn from(s: XRHitTestSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRHitTestSource> for Any {
    fn from(s: &XRHitTestSource) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRHitTestSource);


impl XRHitTestSource {
    /// The cancel method.
    /// [`XRHitTestSource.cancel`](https://developer.mozilla.org/en-US/docs/Web/API/XRHitTestSource/cancel)
    pub fn cancel(&self, ) -> Undefined {
        self.inner.call("cancel", &[]).as_::<Undefined>()
    }
}
