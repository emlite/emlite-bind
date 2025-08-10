use super::*;

/// The BarcodeDetectorOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BarcodeDetectorOptions {
    inner: Any,
}

impl FromVal for BarcodeDetectorOptions {
    fn from_val(v: &Any) -> Self {
        BarcodeDetectorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BarcodeDetectorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BarcodeDetectorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BarcodeDetectorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BarcodeDetectorOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BarcodeDetectorOptions> for Any {
    fn from(s: BarcodeDetectorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BarcodeDetectorOptions> for Any {
    fn from(s: &BarcodeDetectorOptions) -> Any {
        s.inner.clone()
    }
}

impl BarcodeDetectorOptions {
    /// Getter of the `formats` attribute.
    pub fn formats(&self) -> TypedArray<BarcodeFormat> {
        self.inner.get("formats").as_::<TypedArray<BarcodeFormat>>()
    }

    /// Setter of the `formats` attribute.
    pub fn set_formats(&mut self, value: &TypedArray<BarcodeFormat>) {
        self.inner.set("formats", value);
    }
}
