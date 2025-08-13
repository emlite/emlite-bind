use super::*;




/// The CanvasGradient class.
/// [`CanvasGradient`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CanvasGradient {
    inner: Any,
}

impl FromVal for CanvasGradient {
    fn from_val(v: &Any) -> Self {
        CanvasGradient { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CanvasGradient {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CanvasGradient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CanvasGradient {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CanvasGradient {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CanvasGradient> for Any {
    fn from(s: CanvasGradient) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CanvasGradient> for Any {
    fn from(s: &CanvasGradient) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CanvasGradient);


impl CanvasGradient {
    /// The addColorStop method.
    /// [`CanvasGradient.addColorStop`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient/addColorStop)
    pub fn add_color_stop(&self, offset: f64, color: &JsString) -> Undefined {
        self.inner.call("addColorStop", &[offset.into(), color.into(), ]).as_::<Undefined>()
    }
}
