use super::*;




/// The XRTransientInputHitTestResult class.
/// [`XRTransientInputHitTestResult`](https://developer.mozilla.org/en-US/docs/Web/API/XRTransientInputHitTestResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRTransientInputHitTestResult {
    inner: Any,
}

impl FromVal for XRTransientInputHitTestResult {
    fn from_val(v: &Any) -> Self {
        XRTransientInputHitTestResult { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRTransientInputHitTestResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRTransientInputHitTestResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRTransientInputHitTestResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRTransientInputHitTestResult {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRTransientInputHitTestResult> for Any {
    fn from(s: XRTransientInputHitTestResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRTransientInputHitTestResult> for Any {
    fn from(s: &XRTransientInputHitTestResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRTransientInputHitTestResult);


impl XRTransientInputHitTestResult {
    /// Getter of the `inputSource` attribute.
    /// [`XRTransientInputHitTestResult.inputSource`](https://developer.mozilla.org/en-US/docs/Web/API/XRTransientInputHitTestResult/inputSource)
    pub fn input_source(&self) -> XRInputSource {
        self.inner.get("inputSource").as_::<XRInputSource>()
    }

}
impl XRTransientInputHitTestResult {
    /// Getter of the `results` attribute.
    /// [`XRTransientInputHitTestResult.results`](https://developer.mozilla.org/en-US/docs/Web/API/XRTransientInputHitTestResult/results)
    pub fn results(&self) -> TypedArray<XRHitTestResult> {
        self.inner.get("results").as_::<TypedArray<XRHitTestResult>>()
    }

}
