use super::*;

/// The TranslatorCreateCoreOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TranslatorCreateCoreOptions {
    inner: Any,
}

impl FromVal for TranslatorCreateCoreOptions {
    fn from_val(v: &Any) -> Self {
        TranslatorCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TranslatorCreateCoreOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TranslatorCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TranslatorCreateCoreOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TranslatorCreateCoreOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TranslatorCreateCoreOptions> for Any {
    fn from(s: TranslatorCreateCoreOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TranslatorCreateCoreOptions> for Any {
    fn from(s: &TranslatorCreateCoreOptions) -> Any {
        s.inner.clone()
    }
}

impl TranslatorCreateCoreOptions {
    /// Getter of the `sourceLanguage` attribute.
    pub fn source_language(&self) -> JsString {
        self.inner.get("sourceLanguage").as_::<JsString>()
    }

    /// Setter of the `sourceLanguage` attribute.
    pub fn set_source_language(&mut self, value: &JsString) {
        self.inner.set("sourceLanguage", value);
    }
}
impl TranslatorCreateCoreOptions {
    /// Getter of the `targetLanguage` attribute.
    pub fn target_language(&self) -> JsString {
        self.inner.get("targetLanguage").as_::<JsString>()
    }

    /// Setter of the `targetLanguage` attribute.
    pub fn set_target_language(&mut self, value: &JsString) {
        self.inner.set("targetLanguage", value);
    }
}
