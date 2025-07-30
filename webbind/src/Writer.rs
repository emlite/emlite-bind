use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WriterCreateOptions {
    inner: Any,
}
impl FromVal for WriterCreateOptions {
    fn from_val(v: &Any) -> Self {
        WriterCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WriterCreateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WriterCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WriterCreateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WriterCreateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WriterCreateOptions> for Any {
    fn from(s: WriterCreateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WriterCreateOptions> for Any {
    fn from(s: &WriterCreateOptions) -> Any {
        s.inner.clone()
    }
}

impl WriterCreateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl WriterCreateOptions {
    pub fn monitor(&self) -> Function {
        self.inner.get("monitor").as_::<Function>()
    }

    pub fn set_monitor(&mut self, value: &Function) {
        self.inner.set("monitor", value);
    }
}
impl WriterCreateOptions {
    pub fn shared_context(&self) -> JsString {
        self.inner.get("sharedContext").as_::<JsString>()
    }

    pub fn set_shared_context(&mut self, value: &JsString) {
        self.inner.set("sharedContext", value);
    }
}
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
    pub fn tone(&self) -> WriterTone {
        self.inner.get("tone").as_::<WriterTone>()
    }

    pub fn set_tone(&mut self, value: &WriterTone) {
        self.inner.set("tone", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn format(&self) -> WriterFormat {
        self.inner.get("format").as_::<WriterFormat>()
    }

    pub fn set_format(&mut self, value: &WriterFormat) {
        self.inner.set("format", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn length(&self) -> WriterLength {
        self.inner.get("length").as_::<WriterLength>()
    }

    pub fn set_length(&mut self, value: &WriterLength) {
        self.inner.set("length", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn expected_input_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_expected_input_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn expected_context_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_expected_context_languages(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("expectedContextLanguages", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn output_language(&self) -> JsString {
        self.inner.get("outputLanguage").as_::<JsString>()
    }

    pub fn set_output_language(&mut self, value: &JsString) {
        self.inner.set("outputLanguage", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WriterWriteOptions {
    inner: Any,
}
impl FromVal for WriterWriteOptions {
    fn from_val(v: &Any) -> Self {
        WriterWriteOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WriterWriteOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WriterWriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WriterWriteOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WriterWriteOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WriterWriteOptions> for Any {
    fn from(s: WriterWriteOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WriterWriteOptions> for Any {
    fn from(s: &WriterWriteOptions) -> Any {
        s.inner.clone()
    }
}

impl WriterWriteOptions {
    pub fn context(&self) -> JsString {
        self.inner.get("context").as_::<JsString>()
    }

    pub fn set_context(&mut self, value: &JsString) {
        self.inner.set("context", value);
    }
}
impl WriterWriteOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
/// The Writer class.
/// [`Writer`](https://developer.mozilla.org/en-US/docs/Web/API/Writer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Writer {
    inner: Any,
}
impl FromVal for Writer {
    fn from_val(v: &Any) -> Self {
        Writer {
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
impl core::ops::Deref for Writer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Writer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Writer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Writer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Writer> for Any {
    fn from(s: Writer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Writer> for Any {
    fn from(s: &Writer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Writer);

impl Writer {
    /// The create method.
    /// [`Writer.create`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/create)
    pub fn create0() -> Promise<Writer> {
        Any::global("Writer")
            .call("create", &[])
            .as_::<Promise<Writer>>()
    }
    /// The create method.
    /// [`Writer.create`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/create)
    pub fn create1(options: &WriterCreateOptions) -> Promise<Writer> {
        Any::global("Writer")
            .call("create", &[options.into()])
            .as_::<Promise<Writer>>()
    }
}
impl Writer {
    /// The availability method.
    /// [`Writer.availability`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/availability)
    pub fn availability0() -> Promise<Availability> {
        Any::global("Writer")
            .call("availability", &[])
            .as_::<Promise<Availability>>()
    }
    /// The availability method.
    /// [`Writer.availability`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/availability)
    pub fn availability1(options: &WriterCreateCoreOptions) -> Promise<Availability> {
        Any::global("Writer")
            .call("availability", &[options.into()])
            .as_::<Promise<Availability>>()
    }
}
impl Writer {
    /// The write method.
    /// [`Writer.write`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/write)
    pub fn write0(&self, input: &JsString) -> Promise<JsString> {
        self.inner
            .call("write", &[input.into()])
            .as_::<Promise<JsString>>()
    }
    /// The write method.
    /// [`Writer.write`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/write)
    pub fn write1(&self, input: &JsString, options: &WriterWriteOptions) -> Promise<JsString> {
        self.inner
            .call("write", &[input.into(), options.into()])
            .as_::<Promise<JsString>>()
    }
}
impl Writer {
    /// The writeStreaming method.
    /// [`Writer.writeStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/writeStreaming)
    pub fn write_streaming0(&self, input: &JsString) -> ReadableStream {
        self.inner
            .call("writeStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }
    /// The writeStreaming method.
    /// [`Writer.writeStreaming`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/writeStreaming)
    pub fn write_streaming1(
        &self,
        input: &JsString,
        options: &WriterWriteOptions,
    ) -> ReadableStream {
        self.inner
            .call("writeStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Writer {
    /// Getter of the `sharedContext` attribute.
    /// [`Writer.sharedContext`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/sharedContext)
    pub fn shared_context(&self) -> JsString {
        self.inner.get("sharedContext").as_::<JsString>()
    }
}
impl Writer {
    /// Getter of the `tone` attribute.
    /// [`Writer.tone`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/tone)
    pub fn tone(&self) -> WriterTone {
        self.inner.get("tone").as_::<WriterTone>()
    }
}
impl Writer {
    /// Getter of the `format` attribute.
    /// [`Writer.format`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/format)
    pub fn format(&self) -> WriterFormat {
        self.inner.get("format").as_::<WriterFormat>()
    }
}
impl Writer {
    /// Getter of the `length` attribute.
    /// [`Writer.length`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/length)
    pub fn length(&self) -> WriterLength {
        self.inner.get("length").as_::<WriterLength>()
    }
}
impl Writer {
    /// Getter of the `expectedInputLanguages` attribute.
    /// [`Writer.expectedInputLanguages`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/expectedInputLanguages)
    pub fn expected_input_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<TypedArray<JsString>>()
    }
}
impl Writer {
    /// Getter of the `expectedContextLanguages` attribute.
    /// [`Writer.expectedContextLanguages`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/expectedContextLanguages)
    pub fn expected_context_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<TypedArray<JsString>>()
    }
}
impl Writer {
    /// Getter of the `outputLanguage` attribute.
    /// [`Writer.outputLanguage`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/outputLanguage)
    pub fn output_language(&self) -> JsString {
        self.inner.get("outputLanguage").as_::<JsString>()
    }
}
impl Writer {
    /// The measureInputUsage method.
    /// [`Writer.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/measureInputUsage)
    pub fn measure_input_usage0(&self, input: &JsString) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise<f64>>()
    }
    /// The measureInputUsage method.
    /// [`Writer.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/measureInputUsage)
    pub fn measure_input_usage1(
        &self,
        input: &JsString,
        options: &WriterWriteOptions,
    ) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise<f64>>()
    }
}
impl Writer {
    /// Getter of the `inputQuota` attribute.
    /// [`Writer.inputQuota`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/inputQuota)
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Writer {
    /// The destroy method.
    /// [`Writer.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
