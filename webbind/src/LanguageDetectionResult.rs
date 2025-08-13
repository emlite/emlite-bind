use super::*;




/// The LanguageDetectionResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LanguageDetectionResult {
    inner: Any,
}

impl FromVal for LanguageDetectionResult {
    fn from_val(v: &Any) -> Self {
        LanguageDetectionResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for LanguageDetectionResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LanguageDetectionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LanguageDetectionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LanguageDetectionResult {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<LanguageDetectionResult> for Any {
    fn from(s: LanguageDetectionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LanguageDetectionResult> for Any {
    fn from(s: &LanguageDetectionResult) -> Any {
        s.inner.clone()
    }
}

impl LanguageDetectionResult {
    /// Getter of the `detectedLanguage` attribute.
    pub fn detected_language(&self) -> JsString {
        self.inner.get("detectedLanguage").as_::<JsString>()
    }

    /// Setter of the `detectedLanguage` attribute.
    pub fn set_detected_language(&mut self, value: &JsString) {
        self.inner.set("detectedLanguage", value);
    }
}
impl LanguageDetectionResult {
    /// Getter of the `confidence` attribute.
    pub fn confidence(&self) -> f64 {
        self.inner.get("confidence").as_::<f64>()
    }

    /// Setter of the `confidence` attribute.
    pub fn set_confidence(&mut self, value: f64) {
        self.inner.set("confidence", value);
    }
}
