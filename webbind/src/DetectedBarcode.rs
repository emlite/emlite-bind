use super::*;

/// The DetectedBarcode dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DetectedBarcode {
    inner: Any,
}

impl FromVal for DetectedBarcode {
    fn from_val(v: &Any) -> Self {
        DetectedBarcode { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DetectedBarcode {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DetectedBarcode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DetectedBarcode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DetectedBarcode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DetectedBarcode> for Any {
    fn from(s: DetectedBarcode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DetectedBarcode> for Any {
    fn from(s: &DetectedBarcode) -> Any {
        s.inner.clone()
    }
}

impl DetectedBarcode {
    /// Getter of the `boundingBox` attribute.
    pub fn bounding_box(&self) -> DOMRectReadOnly {
        self.inner.get("boundingBox").as_::<DOMRectReadOnly>()
    }

    /// Setter of the `boundingBox` attribute.
    pub fn set_bounding_box(&mut self, value: &DOMRectReadOnly) {
        self.inner.set("boundingBox", value);
    }
}
impl DetectedBarcode {
    /// Getter of the `rawValue` attribute.
    pub fn raw_value(&self) -> JsString {
        self.inner.get("rawValue").as_::<JsString>()
    }

    /// Setter of the `rawValue` attribute.
    pub fn set_raw_value(&mut self, value: &JsString) {
        self.inner.set("rawValue", value);
    }
}
impl DetectedBarcode {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> BarcodeFormat {
        self.inner.get("format").as_::<BarcodeFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &BarcodeFormat) {
        self.inner.set("format", value);
    }
}
impl DetectedBarcode {
    /// Getter of the `cornerPoints` attribute.
    pub fn corner_points(&self) -> TypedArray<Point2D> {
        self.inner.get("cornerPoints").as_::<TypedArray<Point2D>>()
    }

    /// Setter of the `cornerPoints` attribute.
    pub fn set_corner_points(&mut self, value: &TypedArray<Point2D>) {
        self.inner.set("cornerPoints", value);
    }
}
