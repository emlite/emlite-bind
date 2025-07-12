use super::*;

#[derive(Clone, Debug)]
pub struct WriterCreateOptions {
    inner: emlite::Val,
}
impl FromVal for WriterCreateOptions {
    fn from_val(v: &emlite::Val) -> Self {
        WriterCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WriterCreateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WriterCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WriterCreateOptions> for emlite::Val {
    fn from(s: WriterCreateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WriterCreateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl WriterCreateOptions {
    pub fn monitor(&self) -> jsbind::Function {
        self.inner.get("monitor").as_::<jsbind::Function>()
    }

    pub fn set_monitor(&mut self, value: jsbind::Function) {
        self.inner.set("monitor", value);
    }
}
impl WriterCreateOptions {
    pub fn shared_context(&self) -> jsbind::DOMString {
        self.inner.get("sharedContext").as_::<jsbind::DOMString>()
    }

    pub fn set_shared_context(&mut self, value: jsbind::DOMString) {
        self.inner.set("sharedContext", value);
    }
}
#[derive(Clone, Debug)]
pub struct WriterCreateCoreOptions {
    inner: emlite::Val,
}
impl FromVal for WriterCreateCoreOptions {
    fn from_val(v: &emlite::Val) -> Self {
        WriterCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WriterCreateCoreOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WriterCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WriterCreateCoreOptions> for emlite::Val {
    fn from(s: WriterCreateCoreOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WriterCreateCoreOptions {
    pub fn tone(&self) -> WriterTone {
        self.inner.get("tone").as_::<WriterTone>()
    }

    pub fn set_tone(&mut self, value: WriterTone) {
        self.inner.set("tone", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn format(&self) -> WriterFormat {
        self.inner.get("format").as_::<WriterFormat>()
    }

    pub fn set_format(&mut self, value: WriterFormat) {
        self.inner.set("format", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn length(&self) -> WriterLength {
        self.inner.get("length").as_::<WriterLength>()
    }

    pub fn set_length(&mut self, value: WriterLength) {
        self.inner.set("length", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn expected_input_languages(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_expected_input_languages(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn expected_context_languages(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_expected_context_languages(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
        self.inner.set("expectedContextLanguages", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn output_language(&self) -> jsbind::DOMString {
        self.inner.get("outputLanguage").as_::<jsbind::DOMString>()
    }

    pub fn set_output_language(&mut self, value: jsbind::DOMString) {
        self.inner.set("outputLanguage", value);
    }
}
#[derive(Clone, Debug)]
pub struct WriterWriteOptions {
    inner: emlite::Val,
}
impl FromVal for WriterWriteOptions {
    fn from_val(v: &emlite::Val) -> Self {
        WriterWriteOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WriterWriteOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WriterWriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WriterWriteOptions> for emlite::Val {
    fn from(s: WriterWriteOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WriterWriteOptions {
    pub fn context(&self) -> jsbind::DOMString {
        self.inner.get("context").as_::<jsbind::DOMString>()
    }

    pub fn set_context(&mut self, value: jsbind::DOMString) {
        self.inner.set("context", value);
    }
}
impl WriterWriteOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug)]
pub struct Writer {
    inner: emlite::Val,
}
impl FromVal for Writer {
    fn from_val(v: &emlite::Val) -> Self {
        Writer {
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
impl std::ops::Deref for Writer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Writer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Writer> for emlite::Val {
    fn from(s: Writer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Writer {
    pub fn create0() -> jsbind::Promise {
        emlite::Val::global("writer")
            .call("create", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn create1(options: WriterCreateOptions) -> jsbind::Promise {
        emlite::Val::global("writer")
            .call("create", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Writer {
    pub fn availability0() -> jsbind::Promise {
        emlite::Val::global("writer")
            .call("availability", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn availability1(options: WriterCreateCoreOptions) -> jsbind::Promise {
        emlite::Val::global("writer")
            .call("availability", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Writer {
    pub fn write0(&self, input: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("write", &[input.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn write1(&self, input: jsbind::DOMString, options: WriterWriteOptions) -> jsbind::Promise {
        self.inner
            .call("write", &[input.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Writer {
    pub fn write_streaming0(&self, input: jsbind::DOMString) -> ReadableStream {
        self.inner
            .call("writeStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }

    pub fn write_streaming1(
        &self,
        input: jsbind::DOMString,
        options: WriterWriteOptions,
    ) -> ReadableStream {
        self.inner
            .call("writeStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Writer {
    pub fn shared_context(&self) -> jsbind::DOMString {
        self.inner.get("sharedContext").as_::<jsbind::DOMString>()
    }
}
impl Writer {
    pub fn tone(&self) -> WriterTone {
        self.inner.get("tone").as_::<WriterTone>()
    }
}
impl Writer {
    pub fn format(&self) -> WriterFormat {
        self.inner.get("format").as_::<WriterFormat>()
    }
}
impl Writer {
    pub fn length(&self) -> WriterLength {
        self.inner.get("length").as_::<WriterLength>()
    }
}
impl Writer {
    pub fn expected_input_languages(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl Writer {
    pub fn expected_context_languages(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl Writer {
    pub fn output_language(&self) -> jsbind::DOMString {
        self.inner.get("outputLanguage").as_::<jsbind::DOMString>()
    }
}
impl Writer {
    pub fn measure_input_usage0(&self, input: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn measure_input_usage1(
        &self,
        input: jsbind::DOMString,
        options: WriterWriteOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Writer {
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Writer {
    pub fn destroy(&self) -> jsbind::Undefined {
        self.inner.call("destroy", &[]).as_::<jsbind::Undefined>()
    }
}
