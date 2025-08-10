use super::*;

/// The ColorSelectionResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ColorSelectionResult {
    inner: Any,
}

impl FromVal for ColorSelectionResult {
    fn from_val(v: &Any) -> Self {
        ColorSelectionResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ColorSelectionResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ColorSelectionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ColorSelectionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ColorSelectionResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ColorSelectionResult> for Any {
    fn from(s: ColorSelectionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ColorSelectionResult> for Any {
    fn from(s: &ColorSelectionResult) -> Any {
        s.inner.clone()
    }
}

impl ColorSelectionResult {
    /// Getter of the `sRGBHex` attribute.
    pub fn s_rgb_hex(&self) -> JsString {
        self.inner.get("sRGBHex").as_::<JsString>()
    }

    /// Setter of the `sRGBHex` attribute.
    pub fn set_s_rgb_hex(&mut self, value: &JsString) {
        self.inner.set("sRGBHex", value);
    }
}
