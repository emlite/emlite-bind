use super::*;

/// The Translator class.
/// [`Translator`](https://developer.mozilla.org/en-US/docs/Web/API/Translator)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Translator {
    inner: Any,
}

impl FromVal for Translator {
    fn from_val(v: &Any) -> Self {
        Translator {
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

impl core::ops::Deref for Translator {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Translator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Translator {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Translator {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Translator> for Any {
    fn from(s: Translator) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Translator> for Any {
    fn from(s: &Translator) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Translator);

impl Translator {
    /// Getter of the `sourceLanguage` attribute.
    /// [`Translator.sourceLanguage`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/sourceLanguage)
    pub fn source_language(&self) -> JsString {
        self.inner.get("sourceLanguage").as_::<JsString>()
    }
}
impl Translator {
    /// Getter of the `targetLanguage` attribute.
    /// [`Translator.targetLanguage`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/targetLanguage)
    pub fn target_language(&self) -> JsString {
        self.inner.get("targetLanguage").as_::<JsString>()
    }
}
impl Translator {
    /// Getter of the `inputQuota` attribute.
    /// [`Translator.inputQuota`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/inputQuota)
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Translator {
    /// The create method.
    /// [`Translator.create`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/create)
    pub fn create(options: &TranslatorCreateOptions) -> Promise<Translator> {
        Any::global("Translator")
            .call("create", &[options.into()])
            .as_::<Promise<Translator>>()
    }
}
impl Translator {
    /// The availability method.
    /// [`Translator.availability`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/availability)
    pub fn availability(options: &TranslatorCreateCoreOptions) -> Promise<Availability> {
        Any::global("Translator")
            .call("availability", &[options.into()])
            .as_::<Promise<Availability>>()
    }
}
impl Translator {
    /// The translate method.
    /// [`Translator.translate`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/translate)
    pub fn translate0(&self, input: &JsString) -> Promise<JsString> {
        self.inner
            .call("translate", &[input.into()])
            .as_::<Promise<JsString>>()
    }
    /// The translate method.
    /// [`Translator.translate`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/translate)
    pub fn translate1(
        &self,
        input: &JsString,
        options: &TranslatorTranslateOptions,
    ) -> Promise<JsString> {
        self.inner
            .call("translate", &[input.into(), options.into()])
            .as_::<Promise<JsString>>()
    }
}
impl Translator {
    /// The translateStreaming method.
    /// [`Translator.translateStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/translateStreaming)
    pub fn translate_streaming0(&self, input: &JsString) -> ReadableStream {
        self.inner
            .call("translateStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }
    /// The translateStreaming method.
    /// [`Translator.translateStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/translateStreaming)
    pub fn translate_streaming1(
        &self,
        input: &JsString,
        options: &TranslatorTranslateOptions,
    ) -> ReadableStream {
        self.inner
            .call("translateStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Translator {
    /// The measureInputUsage method.
    /// [`Translator.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/measureInputUsage)
    pub fn measure_input_usage0(&self, input: &JsString) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise<f64>>()
    }
    /// The measureInputUsage method.
    /// [`Translator.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/measureInputUsage)
    pub fn measure_input_usage1(
        &self,
        input: &JsString,
        options: &TranslatorTranslateOptions,
    ) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise<f64>>()
    }
}
impl Translator {
    /// The destroy method.
    /// [`Translator.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/Translator/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
