use super::*;

/// The SummarizerCreateCoreOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SummarizerCreateCoreOptions {
    inner: Any,
}

impl FromVal for SummarizerCreateCoreOptions {
    fn from_val(v: &Any) -> Self {
        SummarizerCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SummarizerCreateCoreOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SummarizerCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SummarizerCreateCoreOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SummarizerCreateCoreOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SummarizerCreateCoreOptions> for Any {
    fn from(s: SummarizerCreateCoreOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SummarizerCreateCoreOptions> for Any {
    fn from(s: &SummarizerCreateCoreOptions) -> Any {
        s.inner.clone()
    }
}

impl SummarizerCreateCoreOptions {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> SummarizerType {
        self.inner.get("type").as_::<SummarizerType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &SummarizerType) {
        self.inner.set("type", value);
    }
}
impl SummarizerCreateCoreOptions {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> SummarizerFormat {
        self.inner.get("format").as_::<SummarizerFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &SummarizerFormat) {
        self.inner.set("format", value);
    }
}
impl SummarizerCreateCoreOptions {
    /// Getter of the `length` attribute.
    pub fn length(&self) -> SummarizerLength {
        self.inner.get("length").as_::<SummarizerLength>()
    }

    /// Setter of the `length` attribute.
    pub fn set_length(&mut self, value: &SummarizerLength) {
        self.inner.set("length", value);
    }
}
impl SummarizerCreateCoreOptions {
    /// Getter of the `expectedInputLanguages` attribute.
    pub fn expected_input_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `expectedInputLanguages` attribute.
    pub fn set_expected_input_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
impl SummarizerCreateCoreOptions {
    /// Getter of the `expectedContextLanguages` attribute.
    pub fn expected_context_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `expectedContextLanguages` attribute.
    pub fn set_expected_context_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedContextLanguages", value);
    }
}
impl SummarizerCreateCoreOptions {
    /// Getter of the `outputLanguage` attribute.
    pub fn output_language(&self) -> JsString {
        self.inner.get("outputLanguage").as_::<JsString>()
    }

    /// Setter of the `outputLanguage` attribute.
    pub fn set_output_language(&mut self, value: &JsString) {
        self.inner.set("outputLanguage", value);
    }
}
