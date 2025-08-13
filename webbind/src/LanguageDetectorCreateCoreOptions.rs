use super::*;




/// The LanguageDetectorCreateCoreOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LanguageDetectorCreateCoreOptions {
    inner: Any,
}

impl FromVal for LanguageDetectorCreateCoreOptions {
    fn from_val(v: &Any) -> Self {
        LanguageDetectorCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for LanguageDetectorCreateCoreOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LanguageDetectorCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LanguageDetectorCreateCoreOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LanguageDetectorCreateCoreOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<LanguageDetectorCreateCoreOptions> for Any {
    fn from(s: LanguageDetectorCreateCoreOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LanguageDetectorCreateCoreOptions> for Any {
    fn from(s: &LanguageDetectorCreateCoreOptions) -> Any {
        s.inner.clone()
    }
}

impl LanguageDetectorCreateCoreOptions {
    /// Getter of the `expectedInputLanguages` attribute.
    pub fn expected_input_languages(&self) -> TypedArray<JsString> {
        self.inner.get("expectedInputLanguages").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `expectedInputLanguages` attribute.
    pub fn set_expected_input_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
