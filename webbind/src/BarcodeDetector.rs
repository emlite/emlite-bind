use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DetectedBarcode {
    inner: emlite::Val,
}
impl FromVal for DetectedBarcode {
    fn from_val(v: &emlite::Val) -> Self {
        DetectedBarcode { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DetectedBarcode {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DetectedBarcode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DetectedBarcode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DetectedBarcode {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DetectedBarcode> for emlite::Val {
    fn from(s: DetectedBarcode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DetectedBarcode {
    pub fn bounding_box(&self) -> DOMRectReadOnly {
        self.inner.get("boundingBox").as_::<DOMRectReadOnly>()
    }

    pub fn set_bounding_box(&mut self, value: DOMRectReadOnly) {
        self.inner.set("boundingBox", value);
    }
}
impl DetectedBarcode {
    pub fn raw_value(&self) -> DOMString {
        self.inner.get("rawValue").as_::<DOMString>()
    }

    pub fn set_raw_value(&mut self, value: DOMString) {
        self.inner.set("rawValue", value);
    }
}
impl DetectedBarcode {
    pub fn format(&self) -> BarcodeFormat {
        self.inner.get("format").as_::<BarcodeFormat>()
    }

    pub fn set_format(&mut self, value: BarcodeFormat) {
        self.inner.set("format", value);
    }
}
impl DetectedBarcode {
    pub fn corner_points(&self) -> Sequence<Any> {
        self.inner.get("cornerPoints").as_::<Sequence<Any>>()
    }

    pub fn set_corner_points(&mut self, value: Sequence<Any>) {
        self.inner.set("cornerPoints", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BarcodeDetector {
    inner: emlite::Val,
}
impl FromVal for BarcodeDetector {
    fn from_val(v: &emlite::Val) -> Self {
        BarcodeDetector {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BarcodeDetector {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BarcodeDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BarcodeDetector {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BarcodeDetector {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BarcodeDetector> for emlite::Val {
    fn from(s: BarcodeDetector) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BarcodeDetector);

impl BarcodeDetector {
    pub fn new0() -> BarcodeDetector {
        Self {
            inner: emlite::Val::global("BarcodeDetector")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(barcode_detector_options: Any) -> BarcodeDetector {
        Self {
            inner: emlite::Val::global("BarcodeDetector")
                .new(&[barcode_detector_options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl BarcodeDetector {
    pub fn get_supported_formats() -> Promise {
        emlite::Val::global("BarcodeDetector")
            .call("getSupportedFormats", &[])
            .as_::<Promise>()
    }
}
impl BarcodeDetector {
    pub fn detect(&self, image: Any) -> Promise {
        self.inner.call("detect", &[image.into()]).as_::<Promise>()
    }
}
