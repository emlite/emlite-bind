use super::*;




/// The RewriterCreateCoreOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RewriterCreateCoreOptions {
    inner: Any,
}

impl FromVal for RewriterCreateCoreOptions {
    fn from_val(v: &Any) -> Self {
        RewriterCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RewriterCreateCoreOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RewriterCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RewriterCreateCoreOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RewriterCreateCoreOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RewriterCreateCoreOptions> for Any {
    fn from(s: RewriterCreateCoreOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RewriterCreateCoreOptions> for Any {
    fn from(s: &RewriterCreateCoreOptions) -> Any {
        s.inner.clone()
    }
}

impl RewriterCreateCoreOptions {
    /// Getter of the `tone` attribute.
    pub fn tone(&self) -> RewriterTone {
        self.inner.get("tone").as_::<RewriterTone>()
    }

    /// Setter of the `tone` attribute.
    pub fn set_tone(&mut self, value: &RewriterTone) {
        self.inner.set("tone", value);
    }
}
impl RewriterCreateCoreOptions {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> RewriterFormat {
        self.inner.get("format").as_::<RewriterFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &RewriterFormat) {
        self.inner.set("format", value);
    }
}
impl RewriterCreateCoreOptions {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> RewriterLength {
        self.inner.get("length").as_::<RewriterLength>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: &RewriterLength) {
        self.inner.set("length", value);
    }
}
impl RewriterCreateCoreOptions {
    /// Getter of the `expectedInputLanguages` attribute.
    pub fn expected_input_languages(&self) -> TypedArray<JsString> {
        self.inner.get("expectedInputLanguages").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `expectedInputLanguages` attribute.
    pub fn set_expected_input_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
impl RewriterCreateCoreOptions {
    /// Getter of the `expectedContextLanguages` attribute.
    pub fn expected_context_languages(&self) -> TypedArray<JsString> {
        self.inner.get("expectedContextLanguages").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `expectedContextLanguages` attribute.
    pub fn set_expected_context_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedContextLanguages", value);
    }
}
impl RewriterCreateCoreOptions {
    /// Getter of the `outputLanguage` attribute.
    pub fn output_language(&self) -> JsString {
        self.inner.get("outputLanguage").as_::<JsString>()
    }

    /// Setter of the `outputLanguage` attribute.
    pub fn set_output_language(&mut self, value: &JsString) {
        self.inner.set("outputLanguage", value);
    }
}
