use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RewriterCreateOptions {
    inner: Any,
}
impl FromVal for RewriterCreateOptions {
    fn from_val(v: &Any) -> Self {
        RewriterCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RewriterCreateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RewriterCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RewriterCreateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RewriterCreateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RewriterCreateOptions> for Any {
    fn from(s: RewriterCreateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RewriterCreateOptions> for Any {
    fn from(s: &RewriterCreateOptions) -> Any {
        s.inner.clone()
    }
}

impl RewriterCreateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl RewriterCreateOptions {
    pub fn monitor(&self) -> Function {
        self.inner.get("monitor").as_::<Function>()
    }

    pub fn set_monitor(&mut self, value: &Function) {
        self.inner.set("monitor", value);
    }
}
impl RewriterCreateOptions {
    pub fn shared_context(&self) -> JsString {
        self.inner.get("sharedContext").as_::<JsString>()
    }

    pub fn set_shared_context(&mut self, value: &JsString) {
        self.inner.set("sharedContext", value);
    }
}
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
    pub fn tone(&self) -> RewriterTone {
        self.inner.get("tone").as_::<RewriterTone>()
    }

    pub fn set_tone(&mut self, value: &RewriterTone) {
        self.inner.set("tone", value);
    }
}
impl RewriterCreateCoreOptions {
    pub fn format(&self) -> RewriterFormat {
        self.inner.get("format").as_::<RewriterFormat>()
    }

    pub fn set_format(&mut self, value: &RewriterFormat) {
        self.inner.set("format", value);
    }
}
impl RewriterCreateCoreOptions {
    pub fn length(&self) -> RewriterLength {
        self.inner.get("length").as_::<RewriterLength>()
    }

    pub fn set_length(&mut self, value: &RewriterLength) {
        self.inner.set("length", value);
    }
}
impl RewriterCreateCoreOptions {
    pub fn expected_input_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_expected_input_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
impl RewriterCreateCoreOptions {
    pub fn expected_context_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_expected_context_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedContextLanguages", value);
    }
}
impl RewriterCreateCoreOptions {
    pub fn output_language(&self) -> JsString {
        self.inner.get("outputLanguage").as_::<JsString>()
    }

    pub fn set_output_language(&mut self, value: &JsString) {
        self.inner.set("outputLanguage", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RewriterRewriteOptions {
    inner: Any,
}
impl FromVal for RewriterRewriteOptions {
    fn from_val(v: &Any) -> Self {
        RewriterRewriteOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RewriterRewriteOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RewriterRewriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RewriterRewriteOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RewriterRewriteOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RewriterRewriteOptions> for Any {
    fn from(s: RewriterRewriteOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RewriterRewriteOptions> for Any {
    fn from(s: &RewriterRewriteOptions) -> Any {
        s.inner.clone()
    }
}

impl RewriterRewriteOptions {
    pub fn context(&self) -> JsString {
        self.inner.get("context").as_::<JsString>()
    }

    pub fn set_context(&mut self, value: &JsString) {
        self.inner.set("context", value);
    }
}
impl RewriterRewriteOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
/// The Rewriter class.
/// [`Rewriter`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Rewriter {
    inner: Any,
}
impl FromVal for Rewriter {
    fn from_val(v: &Any) -> Self {
        Rewriter {
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
impl core::ops::Deref for Rewriter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Rewriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Rewriter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Rewriter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Rewriter> for Any {
    fn from(s: Rewriter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Rewriter> for Any {
    fn from(s: &Rewriter) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Rewriter);

impl Rewriter {
    /// The create method.
    /// [`Rewriter.create`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/create)
    pub fn create0() -> Promise<Rewriter> {
        Any::global("Rewriter")
            .call("create", &[])
            .as_::<Promise<Rewriter>>()
    }
    /// The create method.
    /// [`Rewriter.create`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/create)
    pub fn create1(options: &RewriterCreateOptions) -> Promise<Rewriter> {
        Any::global("Rewriter")
            .call("create", &[options.into()])
            .as_::<Promise<Rewriter>>()
    }
}
impl Rewriter {
    /// The availability method.
    /// [`Rewriter.availability`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/availability)
    pub fn availability0() -> Promise<Availability> {
        Any::global("Rewriter")
            .call("availability", &[])
            .as_::<Promise<Availability>>()
    }
    /// The availability method.
    /// [`Rewriter.availability`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/availability)
    pub fn availability1(options: &RewriterCreateCoreOptions) -> Promise<Availability> {
        Any::global("Rewriter")
            .call("availability", &[options.into()])
            .as_::<Promise<Availability>>()
    }
}
impl Rewriter {
    /// The rewrite method.
    /// [`Rewriter.rewrite`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/rewrite)
    pub fn rewrite0(&self, input: &JsString) -> Promise<JsString> {
        self.inner
            .call("rewrite", &[input.into()])
            .as_::<Promise<JsString>>()
    }
    /// The rewrite method.
    /// [`Rewriter.rewrite`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/rewrite)
    pub fn rewrite1(
        &self,
        input: &JsString,
        options: &RewriterRewriteOptions,
    ) -> Promise<JsString> {
        self.inner
            .call("rewrite", &[input.into(), options.into()])
            .as_::<Promise<JsString>>()
    }
}
impl Rewriter {
    /// The rewriteStreaming method.
    /// [`Rewriter.rewriteStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/rewriteStreaming)
    pub fn rewrite_streaming0(&self, input: &JsString) -> ReadableStream {
        self.inner
            .call("rewriteStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }
    /// The rewriteStreaming method.
    /// [`Rewriter.rewriteStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/rewriteStreaming)
    pub fn rewrite_streaming1(
        &self,
        input: &JsString,
        options: &RewriterRewriteOptions,
    ) -> ReadableStream {
        self.inner
            .call("rewriteStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Rewriter {
    /// Getter of the `sharedContext` attribute.
    /// [`Rewriter.sharedContext`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/sharedContext)
    pub fn shared_context(&self) -> JsString {
        self.inner.get("sharedContext").as_::<JsString>()
    }
}
impl Rewriter {
    /// Getter of the `tone` attribute.
    /// [`Rewriter.tone`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/tone)
    pub fn tone(&self) -> RewriterTone {
        self.inner.get("tone").as_::<RewriterTone>()
    }
}
impl Rewriter {
    /// Getter of the `format` attribute.
    /// [`Rewriter.format`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/format)
    pub fn format(&self) -> RewriterFormat {
        self.inner.get("format").as_::<RewriterFormat>()
    }
}
impl Rewriter {
    /// Getter of the `length` attribute.
    /// [`Rewriter.length`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/length)
    pub fn length(&self) -> RewriterLength {
        self.inner.get("length").as_::<RewriterLength>()
    }
}
impl Rewriter {
    /// Getter of the `expectedInputLanguages` attribute.
    /// [`Rewriter.expectedInputLanguages`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/expectedInputLanguages)
    pub fn expected_input_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<TypedArray<JsString>>()
    }
}
impl Rewriter {
    /// Getter of the `expectedContextLanguages` attribute.
    /// [`Rewriter.expectedContextLanguages`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/expectedContextLanguages)
    pub fn expected_context_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<TypedArray<JsString>>()
    }
}
impl Rewriter {
    /// Getter of the `outputLanguage` attribute.
    /// [`Rewriter.outputLanguage`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/outputLanguage)
    pub fn output_language(&self) -> JsString {
        self.inner.get("outputLanguage").as_::<JsString>()
    }
}
impl Rewriter {
    /// The measureInputUsage method.
    /// [`Rewriter.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/measureInputUsage)
    pub fn measure_input_usage0(&self, input: &JsString) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise<f64>>()
    }
    /// The measureInputUsage method.
    /// [`Rewriter.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/measureInputUsage)
    pub fn measure_input_usage1(
        &self,
        input: &JsString,
        options: &RewriterRewriteOptions,
    ) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise<f64>>()
    }
}
impl Rewriter {
    /// Getter of the `inputQuota` attribute.
    /// [`Rewriter.inputQuota`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/inputQuota)
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Rewriter {
    /// The destroy method.
    /// [`Rewriter.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/Rewriter/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
