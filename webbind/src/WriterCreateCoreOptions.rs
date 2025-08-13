use super::*;




/// The WriterCreateCoreOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WriterCreateCoreOptions {
    inner: Any,
}

impl FromVal for WriterCreateCoreOptions {
    fn from_val(v: &Any) -> Self {
        WriterCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WriterCreateCoreOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WriterCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WriterCreateCoreOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WriterCreateCoreOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WriterCreateCoreOptions> for Any {
    fn from(s: WriterCreateCoreOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WriterCreateCoreOptions> for Any {
    fn from(s: &WriterCreateCoreOptions) -> Any {
        s.inner.clone()
    }
}

impl WriterCreateCoreOptions {
    /// Getter of the `tone` attribute.
    pub fn tone(&self) -> WriterTone {
        self.inner.get("tone").as_::<WriterTone>()
    }

    /// Setter of the `tone` attribute.
    pub fn set_tone(&mut self, value: &WriterTone) {
        self.inner.set("tone", value);
    }
}
impl WriterCreateCoreOptions {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> WriterFormat {
        self.inner.get("format").as_::<WriterFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &WriterFormat) {
        self.inner.set("format", value);
    }
}
impl WriterCreateCoreOptions {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> WriterLength {
        self.inner.get("length").as_::<WriterLength>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: &WriterLength) {
        self.inner.set("length", value);
    }
}
impl WriterCreateCoreOptions {
    /// Getter of the `expectedInputLanguages` attribute.
    pub fn expected_input_languages(&self) -> TypedArray<JsString> {
        self.inner.get("expectedInputLanguages").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `expectedInputLanguages` attribute.
    pub fn set_expected_input_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
impl WriterCreateCoreOptions {
    /// Getter of the `expectedContextLanguages` attribute.
    pub fn expected_context_languages(&self) -> TypedArray<JsString> {
        self.inner.get("expectedContextLanguages").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `expectedContextLanguages` attribute.
    pub fn set_expected_context_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedContextLanguages", value);
    }
}
impl WriterCreateCoreOptions {
    /// Getter of the `outputLanguage` attribute.
    pub fn output_language(&self) -> JsString {
        self.inner.get("outputLanguage").as_::<JsString>()
    }

    /// Setter of the `outputLanguage` attribute.
    pub fn set_output_language(&mut self, value: &JsString) {
        self.inner.set("outputLanguage", value);
    }
}
