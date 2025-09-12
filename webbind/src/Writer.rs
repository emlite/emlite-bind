use super::*;

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
    /// Getter of the `inputQuota` attribute.
    /// [`Writer.inputQuota`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/inputQuota)
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
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
    /// The destroy method.
    /// [`Writer.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/Writer/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
