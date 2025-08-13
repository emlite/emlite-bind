use super::*;




/// The DetectedText dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DetectedText {
    inner: Any,
}

impl FromVal for DetectedText {
    fn from_val(v: &Any) -> Self {
        DetectedText { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DetectedText {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DetectedText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DetectedText {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DetectedText {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DetectedText> for Any {
    fn from(s: DetectedText) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DetectedText> for Any {
    fn from(s: &DetectedText) -> Any {
        s.inner.clone()
    }
}

impl DetectedText {
    /// Getter of the `boundingBox` attribute.
    pub fn bounding_box(&self) -> DOMRectReadOnly {
        self.inner.get("boundingBox").as_::<DOMRectReadOnly>()
    }

    /// Setter of the `boundingBox` attribute.
    pub fn set_bounding_box(&mut self, value: &DOMRectReadOnly) {
        self.inner.set("boundingBox", value);
    }
}
impl DetectedText {
    /// Getter of the `rawValue` attribute.
    pub fn raw_value(&self) -> JsString {
        self.inner.get("rawValue").as_::<JsString>()
    }

    /// Setter of the `rawValue` attribute.
    pub fn set_raw_value(&mut self, value: &JsString) {
        self.inner.set("rawValue", value);
    }
}
impl DetectedText {
    /// Getter of the `cornerPoints` attribute.
    pub fn corner_points(&self) -> TypedArray<Point2D> {
        self.inner.get("cornerPoints").as_::<TypedArray<Point2D>>()
    }

    /// Setter of the `cornerPoints` attribute.
    pub fn set_corner_points(&mut self, value: &TypedArray<Point2D>) {
        self.inner.set("cornerPoints", value);
    }
}
