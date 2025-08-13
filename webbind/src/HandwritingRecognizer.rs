use super::*;




/// The HandwritingRecognizer class.
/// [`HandwritingRecognizer`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingRecognizer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingRecognizer {
    inner: Any,
}

impl FromVal for HandwritingRecognizer {
    fn from_val(v: &Any) -> Self {
        HandwritingRecognizer { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HandwritingRecognizer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HandwritingRecognizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HandwritingRecognizer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HandwritingRecognizer {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HandwritingRecognizer> for Any {
    fn from(s: HandwritingRecognizer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HandwritingRecognizer> for Any {
    fn from(s: &HandwritingRecognizer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(HandwritingRecognizer);


impl HandwritingRecognizer {
    /// The startDrawing method.
    /// [`HandwritingRecognizer.startDrawing`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingRecognizer/startDrawing)
    pub fn start_drawing0(&self, ) -> HandwritingDrawing {
        self.inner.call("startDrawing", &[]).as_::<HandwritingDrawing>()
    }
    /// The startDrawing method.
    /// [`HandwritingRecognizer.startDrawing`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingRecognizer/startDrawing)
    pub fn start_drawing1(&self, hints: &HandwritingHints) -> HandwritingDrawing {
        self.inner.call("startDrawing", &[hints.into(), ]).as_::<HandwritingDrawing>()
    }
}
impl HandwritingRecognizer {
    /// The finish method.
    /// [`HandwritingRecognizer.finish`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingRecognizer/finish)
    pub fn finish(&self, ) -> Undefined {
        self.inner.call("finish", &[]).as_::<Undefined>()
    }
}
