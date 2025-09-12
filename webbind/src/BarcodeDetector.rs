use super::*;

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
    pub fn new() -> BarcodeDetector {
        Self {
            inner: Any::global("BarcodeDetector").new(&[]).as_::<Any>(),
        }
    }
}

impl BarcodeDetector {
    /// The `new BarcodeDetector(..)` constructor, creating a new BarcodeDetector instance
    pub fn new_with_barcode_detector_options(
        barcode_detector_options: &BarcodeDetectorOptions,
    ) -> BarcodeDetector {
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
    pub fn get_supported_formats() -> Promise<TypedArray<BarcodeFormat>> {
        Any::global("BarcodeDetector")
            .call("getSupportedFormats", &[])
            .as_::<Promise<TypedArray<BarcodeFormat>>>()
    }
}
impl BarcodeDetector {
    /// The detect method.
    /// [`BarcodeDetector.detect`](https://developer.mozilla.org/en-US/docs/Web/API/BarcodeDetector/detect)
    pub fn detect(&self, image: &Any) -> Promise<TypedArray<DetectedBarcode>> {
        self.inner
            .call("detect", &[image.into()])
            .as_::<Promise<TypedArray<DetectedBarcode>>>()
    }
}
