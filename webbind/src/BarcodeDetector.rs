use super::*;

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
    pub fn bounding_box(&self) -> DOMRectReadOnly {
        self.inner.get("boundingBox").as_::<DOMRectReadOnly>()
    }

    pub fn set_bounding_box(&mut self, value: &DOMRectReadOnly) {
        self.inner.set("boundingBox", value);
    }
}
impl DetectedBarcode {
    pub fn raw_value(&self) -> DOMString {
        self.inner.get("rawValue").as_::<DOMString>()
    }

    pub fn set_raw_value(&mut self, value: &DOMString) {
        self.inner.set("rawValue", value);
    }
}
impl DetectedBarcode {
    pub fn format(&self) -> BarcodeFormat {
        self.inner.get("format").as_::<BarcodeFormat>()
    }

    pub fn set_format(&mut self, value: &BarcodeFormat) {
        self.inner.set("format", value);
    }
}
impl DetectedBarcode {
    pub fn corner_points(&self) -> Sequence<Any> {
        self.inner.get("cornerPoints").as_::<Sequence<Any>>()
    }

    pub fn set_corner_points(&mut self, value: &Sequence<Any>) {
        self.inner.set("cornerPoints", value);
    }
}
/// The BarcodeDetector class.
/// [`BarcodeDetector`](https://developer.mozilla.org/en-US/docs/Web/API/BarcodeDetector)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BarcodeDetector {
    inner: Any,
}
impl FromVal for BarcodeDetector {
    fn from_val(v: &Any) -> Self {
        BarcodeDetector {
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
impl core::ops::Deref for BarcodeDetector {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BarcodeDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BarcodeDetector {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BarcodeDetector {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BarcodeDetector> for Any {
    fn from(s: BarcodeDetector) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BarcodeDetector> for Any {
    fn from(s: &BarcodeDetector) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BarcodeDetector);

impl BarcodeDetector {
    /// The `new BarcodeDetector(..)` constructor, creating a new BarcodeDetector instance
    pub fn new0() -> BarcodeDetector {
        Self {
            inner: Any::global("BarcodeDetector").new(&[]).as_::<Any>(),
        }
    }

    /// The `new BarcodeDetector(..)` constructor, creating a new BarcodeDetector instance
    pub fn new1(barcode_detector_options: &Any) -> BarcodeDetector {
        Self {
            inner: Any::global("BarcodeDetector")
                .new(&[barcode_detector_options.into()])
                .as_::<Any>(),
        }
    }
}
impl BarcodeDetector {
    /// The getSupportedFormats method.
    /// [`BarcodeDetector.getSupportedFormats`](https://developer.mozilla.org/en-US/docs/Web/API/BarcodeDetector/getSupportedFormats)
    pub fn get_supported_formats() -> Promise<Sequence<BarcodeFormat>> {
        Any::global("BarcodeDetector")
            .call("getSupportedFormats", &[])
            .as_::<Promise<Sequence<BarcodeFormat>>>()
    }
}
impl BarcodeDetector {
    /// The detect method.
    /// [`BarcodeDetector.detect`](https://developer.mozilla.org/en-US/docs/Web/API/BarcodeDetector/detect)
    pub fn detect(&self, image: &Any) -> Promise<Sequence<DetectedBarcode>> {
        self.inner
            .call("detect", &[image.into()])
            .as_::<Promise<Sequence<DetectedBarcode>>>()
    }
}
