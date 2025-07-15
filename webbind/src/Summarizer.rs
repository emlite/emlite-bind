use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SummarizerCreateOptions {
    inner: emlite::Val,
}
impl FromVal for SummarizerCreateOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SummarizerCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SummarizerCreateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SummarizerCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SummarizerCreateOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SummarizerCreateOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SummarizerCreateOptions> for emlite::Val {
    fn from(s: SummarizerCreateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SummarizerCreateOptions> for emlite::Val {
    fn from(s: &SummarizerCreateOptions) -> emlite::Val {
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
    pub fn shared_context(&self) -> String {
        self.inner.get("sharedContext").as_::<String>()
    }

    pub fn set_shared_context(&mut self, value: &str) {
        self.inner.set("sharedContext", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SummarizerCreateCoreOptions {
    inner: emlite::Val,
}
impl FromVal for SummarizerCreateCoreOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SummarizerCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SummarizerCreateCoreOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SummarizerCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SummarizerCreateCoreOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SummarizerCreateCoreOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SummarizerCreateCoreOptions> for emlite::Val {
    fn from(s: SummarizerCreateCoreOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SummarizerCreateCoreOptions> for emlite::Val {
    fn from(s: &SummarizerCreateCoreOptions) -> emlite::Val {
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
    pub fn expected_input_languages(&self) -> Sequence<String> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<Sequence<String>>()
    }

    pub fn set_expected_input_languages(&mut self, value: &Sequence<String>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
impl SummarizerCreateCoreOptions {
    pub fn expected_context_languages(&self) -> Sequence<String> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<Sequence<String>>()
    }

    pub fn set_expected_context_languages(&mut self, value: &Sequence<String>) {
        self.inner.set("expectedContextLanguages", value);
    }
}
impl SummarizerCreateCoreOptions {
    pub fn output_language(&self) -> String {
        self.inner.get("outputLanguage").as_::<String>()
    }

    pub fn set_output_language(&mut self, value: &str) {
        self.inner.set("outputLanguage", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SummarizerSummarizeOptions {
    inner: emlite::Val,
}
impl FromVal for SummarizerSummarizeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SummarizerSummarizeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SummarizerSummarizeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SummarizerSummarizeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SummarizerSummarizeOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SummarizerSummarizeOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SummarizerSummarizeOptions> for emlite::Val {
    fn from(s: SummarizerSummarizeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SummarizerSummarizeOptions> for emlite::Val {
    fn from(s: &SummarizerSummarizeOptions) -> emlite::Val {
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
    pub fn context(&self) -> String {
        self.inner.get("context").as_::<String>()
    }

    pub fn set_context(&mut self, value: &str) {
        self.inner.set("context", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Summarizer {
    inner: emlite::Val,
}
impl FromVal for Summarizer {
    fn from_val(v: &emlite::Val) -> Self {
        Summarizer {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Summarizer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Summarizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Summarizer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Summarizer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Summarizer> for emlite::Val {
    fn from(s: Summarizer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Summarizer> for emlite::Val {
    fn from(s: &Summarizer) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Summarizer);

impl Summarizer {
    pub fn create0() -> Promise {
        emlite::Val::global("Summarizer")
            .call("create", &[])
            .as_::<Promise>()
    }

    pub fn create1(options: &SummarizerCreateOptions) -> Promise {
        emlite::Val::global("Summarizer")
            .call("create", &[options.into()])
            .as_::<Promise>()
    }
}
impl Summarizer {
    pub fn availability0() -> Promise {
        emlite::Val::global("Summarizer")
            .call("availability", &[])
            .as_::<Promise>()
    }

    pub fn availability1(options: &SummarizerCreateCoreOptions) -> Promise {
        emlite::Val::global("Summarizer")
            .call("availability", &[options.into()])
            .as_::<Promise>()
    }
}
impl Summarizer {
    pub fn summarize0(&self, input: &str) -> Promise {
        self.inner
            .call("summarize", &[input.into()])
            .as_::<Promise>()
    }

    pub fn summarize1(&self, input: &str, options: &SummarizerSummarizeOptions) -> Promise {
        self.inner
            .call("summarize", &[input.into(), options.into()])
            .as_::<Promise>()
    }
}
impl Summarizer {
    pub fn summarize_streaming0(&self, input: &str) -> ReadableStream {
        self.inner
            .call("summarizeStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }

    pub fn summarize_streaming1(
        &self,
        input: &str,
        options: &SummarizerSummarizeOptions,
    ) -> ReadableStream {
        self.inner
            .call("summarizeStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Summarizer {
    pub fn shared_context(&self) -> String {
        self.inner.get("sharedContext").as_::<String>()
    }
}
impl Summarizer {
    pub fn type_(&self) -> SummarizerType {
        self.inner.get("type").as_::<SummarizerType>()
    }
}
impl Summarizer {
    pub fn format(&self) -> SummarizerFormat {
        self.inner.get("format").as_::<SummarizerFormat>()
    }
}
impl Summarizer {
    pub fn length(&self) -> SummarizerLength {
        self.inner.get("length").as_::<SummarizerLength>()
    }
}
impl Summarizer {
    pub fn expected_input_languages(&self) -> FrozenArray<String> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<FrozenArray<String>>()
    }
}
impl Summarizer {
    pub fn expected_context_languages(&self) -> FrozenArray<String> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<FrozenArray<String>>()
    }
}
impl Summarizer {
    pub fn output_language(&self) -> String {
        self.inner.get("outputLanguage").as_::<String>()
    }
}
impl Summarizer {
    pub fn measure_input_usage0(&self, input: &str) -> Promise {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise>()
    }

    pub fn measure_input_usage1(
        &self,
        input: &str,
        options: &SummarizerSummarizeOptions,
    ) -> Promise {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise>()
    }
}
impl Summarizer {
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Summarizer {
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
