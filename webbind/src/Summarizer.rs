use super::*;

/// The Summarizer class.
/// [`Summarizer`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Summarizer {
    inner: Any,
}

impl FromVal for Summarizer {
    fn from_val(v: &Any) -> Self {
        Summarizer {
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

impl core::ops::Deref for Summarizer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Summarizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Summarizer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Summarizer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Summarizer> for Any {
    fn from(s: Summarizer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Summarizer> for Any {
    fn from(s: &Summarizer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Summarizer);

impl Summarizer {
    /// Getter of the `sharedContext` attribute.
    /// [`Summarizer.sharedContext`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/sharedContext)
    pub fn shared_context(&self) -> JsString {
        self.inner.get("sharedContext").as_::<JsString>()
    }
}
impl Summarizer {
    /// Getter of the `type` attribute.
    /// [`Summarizer.type`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/type)
    pub fn type_(&self) -> SummarizerType {
        self.inner.get("type").as_::<SummarizerType>()
    }
}
impl Summarizer {
    /// Getter of the `format` attribute.
    /// [`Summarizer.format`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/format)
    pub fn format(&self) -> SummarizerFormat {
        self.inner.get("format").as_::<SummarizerFormat>()
    }
}
impl Summarizer {
    /// Getter of the `length` attribute.
    /// [`Summarizer.length`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/length)
    pub fn length(&self) -> SummarizerLength {
        self.inner.get("length").as_::<SummarizerLength>()
    }
}
impl Summarizer {
    /// Getter of the `expectedInputLanguages` attribute.
    /// [`Summarizer.expectedInputLanguages`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/expectedInputLanguages)
    pub fn expected_input_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<TypedArray<JsString>>()
    }
}
impl Summarizer {
    /// Getter of the `expectedContextLanguages` attribute.
    /// [`Summarizer.expectedContextLanguages`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/expectedContextLanguages)
    pub fn expected_context_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<TypedArray<JsString>>()
    }
}
impl Summarizer {
    /// Getter of the `outputLanguage` attribute.
    /// [`Summarizer.outputLanguage`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/outputLanguage)
    pub fn output_language(&self) -> JsString {
        self.inner.get("outputLanguage").as_::<JsString>()
    }
}
impl Summarizer {
    /// Getter of the `inputQuota` attribute.
    /// [`Summarizer.inputQuota`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/inputQuota)
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Summarizer {
    /// The create method.
    /// [`Summarizer.create`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/create)
    pub fn create0() -> Promise<Summarizer> {
        Any::global("Summarizer")
            .call("create", &[])
            .as_::<Promise<Summarizer>>()
    }
    /// The create method.
    /// [`Summarizer.create`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/create)
    pub fn create1(options: &SummarizerCreateOptions) -> Promise<Summarizer> {
        Any::global("Summarizer")
            .call("create", &[options.into()])
            .as_::<Promise<Summarizer>>()
    }
}
impl Summarizer {
    /// The availability method.
    /// [`Summarizer.availability`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/availability)
    pub fn availability0() -> Promise<Availability> {
        Any::global("Summarizer")
            .call("availability", &[])
            .as_::<Promise<Availability>>()
    }
    /// The availability method.
    /// [`Summarizer.availability`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/availability)
    pub fn availability1(options: &SummarizerCreateCoreOptions) -> Promise<Availability> {
        Any::global("Summarizer")
            .call("availability", &[options.into()])
            .as_::<Promise<Availability>>()
    }
}
impl Summarizer {
    /// The summarize method.
    /// [`Summarizer.summarize`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/summarize)
    pub fn summarize0(&self, input: &JsString) -> Promise<JsString> {
        self.inner
            .call("summarize", &[input.into()])
            .as_::<Promise<JsString>>()
    }
    /// The summarize method.
    /// [`Summarizer.summarize`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/summarize)
    pub fn summarize1(
        &self,
        input: &JsString,
        options: &SummarizerSummarizeOptions,
    ) -> Promise<JsString> {
        self.inner
            .call("summarize", &[input.into(), options.into()])
            .as_::<Promise<JsString>>()
    }
}
impl Summarizer {
    /// The summarizeStreaming method.
    /// [`Summarizer.summarizeStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/summarizeStreaming)
    pub fn summarize_streaming0(&self, input: &JsString) -> ReadableStream {
        self.inner
            .call("summarizeStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }
    /// The summarizeStreaming method.
    /// [`Summarizer.summarizeStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/summarizeStreaming)
    pub fn summarize_streaming1(
        &self,
        input: &JsString,
        options: &SummarizerSummarizeOptions,
    ) -> ReadableStream {
        self.inner
            .call("summarizeStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Summarizer {
    /// The measureInputUsage method.
    /// [`Summarizer.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/measureInputUsage)
    pub fn measure_input_usage0(&self, input: &JsString) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise<f64>>()
    }
    /// The measureInputUsage method.
    /// [`Summarizer.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/measureInputUsage)
    pub fn measure_input_usage1(
        &self,
        input: &JsString,
        options: &SummarizerSummarizeOptions,
    ) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise<f64>>()
    }
}
impl Summarizer {
    /// The destroy method.
    /// [`Summarizer.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
