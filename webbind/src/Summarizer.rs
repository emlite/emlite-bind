use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SummarizerCreateOptions {
    inner: Any,
}
impl FromVal for SummarizerCreateOptions {
    fn from_val(v: &Any) -> Self {
        SummarizerCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SummarizerCreateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SummarizerCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SummarizerCreateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SummarizerCreateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SummarizerCreateOptions> for Any {
    fn from(s: SummarizerCreateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SummarizerCreateOptions> for Any {
    fn from(s: &SummarizerCreateOptions) -> Any {
        s.inner.clone()
    }
}

impl SummarizerCreateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl SummarizerCreateOptions {
    pub fn monitor(&self) -> Function {
        self.inner.get("monitor").as_::<Function>()
    }

    pub fn set_monitor(&mut self, value: &Function) {
        self.inner.set("monitor", value);
    }
}
impl SummarizerCreateOptions {
    pub fn shared_context(&self) -> DOMString {
        self.inner.get("sharedContext").as_::<DOMString>()
    }

    pub fn set_shared_context(&mut self, value: &DOMString) {
        self.inner.set("sharedContext", value);
    }
}
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
    pub fn type_(&self) -> SummarizerType {
        self.inner.get("type").as_::<SummarizerType>()
    }

    pub fn set_type_(&mut self, value: &SummarizerType) {
        self.inner.set("type", value);
    }
}
impl SummarizerCreateCoreOptions {
    pub fn format(&self) -> SummarizerFormat {
        self.inner.get("format").as_::<SummarizerFormat>()
    }

    pub fn set_format(&mut self, value: &SummarizerFormat) {
        self.inner.set("format", value);
    }
}
impl SummarizerCreateCoreOptions {
    pub fn length(&self) -> SummarizerLength {
        self.inner.get("length").as_::<SummarizerLength>()
    }

    pub fn set_length(&mut self, value: &SummarizerLength) {
        self.inner.set("length", value);
    }
}
impl SummarizerCreateCoreOptions {
    pub fn expected_input_languages(&self) -> Sequence<DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<Sequence<DOMString>>()
    }

    pub fn set_expected_input_languages(&mut self, value: &Sequence<DOMString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
impl SummarizerCreateCoreOptions {
    pub fn expected_context_languages(&self) -> Sequence<DOMString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<Sequence<DOMString>>()
    }

    pub fn set_expected_context_languages(&mut self, value: &Sequence<DOMString>) {
        self.inner.set("expectedContextLanguages", value);
    }
}
impl SummarizerCreateCoreOptions {
    pub fn output_language(&self) -> DOMString {
        self.inner.get("outputLanguage").as_::<DOMString>()
    }

    pub fn set_output_language(&mut self, value: &DOMString) {
        self.inner.set("outputLanguage", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SummarizerSummarizeOptions {
    inner: Any,
}
impl FromVal for SummarizerSummarizeOptions {
    fn from_val(v: &Any) -> Self {
        SummarizerSummarizeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SummarizerSummarizeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SummarizerSummarizeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SummarizerSummarizeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SummarizerSummarizeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SummarizerSummarizeOptions> for Any {
    fn from(s: SummarizerSummarizeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SummarizerSummarizeOptions> for Any {
    fn from(s: &SummarizerSummarizeOptions) -> Any {
        s.inner.clone()
    }
}

impl SummarizerSummarizeOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl SummarizerSummarizeOptions {
    pub fn context(&self) -> DOMString {
        self.inner.get("context").as_::<DOMString>()
    }

    pub fn set_context(&mut self, value: &DOMString) {
        self.inner.set("context", value);
    }
}
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
    pub fn summarize0(&self, input: &DOMString) -> Promise<DOMString> {
        self.inner
            .call("summarize", &[input.into()])
            .as_::<Promise<DOMString>>()
    }
    /// The summarize method.
    /// [`Summarizer.summarize`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/summarize)
    pub fn summarize1(
        &self,
        input: &DOMString,
        options: &SummarizerSummarizeOptions,
    ) -> Promise<DOMString> {
        self.inner
            .call("summarize", &[input.into(), options.into()])
            .as_::<Promise<DOMString>>()
    }
}
impl Summarizer {
    /// The summarizeStreaming method.
    /// [`Summarizer.summarizeStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/summarizeStreaming)
    pub fn summarize_streaming0(&self, input: &DOMString) -> ReadableStream {
        self.inner
            .call("summarizeStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }
    /// The summarizeStreaming method.
    /// [`Summarizer.summarizeStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/summarizeStreaming)
    pub fn summarize_streaming1(
        &self,
        input: &DOMString,
        options: &SummarizerSummarizeOptions,
    ) -> ReadableStream {
        self.inner
            .call("summarizeStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Summarizer {
    /// Getter of the `sharedContext` attribute.
    /// [`Summarizer.sharedContext`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/sharedContext)
    pub fn shared_context(&self) -> DOMString {
        self.inner.get("sharedContext").as_::<DOMString>()
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
    pub fn expected_input_languages(&self) -> FrozenArray<DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<FrozenArray<DOMString>>()
    }
}
impl Summarizer {
    /// Getter of the `expectedContextLanguages` attribute.
    /// [`Summarizer.expectedContextLanguages`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/expectedContextLanguages)
    pub fn expected_context_languages(&self) -> FrozenArray<DOMString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<FrozenArray<DOMString>>()
    }
}
impl Summarizer {
    /// Getter of the `outputLanguage` attribute.
    /// [`Summarizer.outputLanguage`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/outputLanguage)
    pub fn output_language(&self) -> DOMString {
        self.inner.get("outputLanguage").as_::<DOMString>()
    }
}
impl Summarizer {
    /// The measureInputUsage method.
    /// [`Summarizer.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/measureInputUsage)
    pub fn measure_input_usage0(&self, input: &DOMString) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise<f64>>()
    }
    /// The measureInputUsage method.
    /// [`Summarizer.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/measureInputUsage)
    pub fn measure_input_usage1(
        &self,
        input: &DOMString,
        options: &SummarizerSummarizeOptions,
    ) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise<f64>>()
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
    /// The destroy method.
    /// [`Summarizer.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/Summarizer/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
