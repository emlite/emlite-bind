use super::*;




/// The HandwritingPrediction dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingPrediction {
    inner: Any,
}

impl FromVal for HandwritingPrediction {
    fn from_val(v: &Any) -> Self {
        HandwritingPrediction { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HandwritingPrediction {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HandwritingPrediction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HandwritingPrediction {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HandwritingPrediction {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HandwritingPrediction> for Any {
    fn from(s: HandwritingPrediction) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HandwritingPrediction> for Any {
    fn from(s: &HandwritingPrediction) -> Any {
        s.inner.clone()
    }
}

impl HandwritingPrediction {
    /// Getter of the `text` attribute.
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }

    /// Setter of the `text` attribute.
    pub fn set_text(&mut self, value: &JsString) {
        self.inner.set("text", value);
    }
}
impl HandwritingPrediction {
    /// Getter of the `segmentationResult` attribute.
    pub fn segmentation_result(&self) -> TypedArray<HandwritingSegment> {
        self.inner.get("segmentationResult").as_::<TypedArray<HandwritingSegment>>()
    }

    /// Setter of the `segmentationResult` attribute.
    pub fn set_segmentation_result(&mut self, value: &TypedArray<HandwritingSegment>) {
        self.inner.set("segmentationResult", value);
    }
}
