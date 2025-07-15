use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for WriterCreateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WriterCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WriterCreateOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WriterCreateOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WriterCreateOptions> for emlite::Val {
    fn from(s: WriterCreateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WriterCreateOptions> for emlite::Val {
    fn from(s: &WriterCreateOptions) -> emlite::Val {
        s.inner.clone()
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
    pub fn monitor(&self) -> Function {
        self.inner.get("monitor").as_::<Function>()
    }

    pub fn set_monitor(&mut self, value: Function) {
        self.inner.set("monitor", value);
    }
}
impl WriterCreateOptions {
    pub fn shared_context(&self) -> DOMString {
        self.inner.get("sharedContext").as_::<DOMString>()
    }

    pub fn set_shared_context(&mut self, value: DOMString) {
        self.inner.set("sharedContext", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for WriterCreateCoreOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WriterCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WriterCreateCoreOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WriterCreateCoreOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WriterCreateCoreOptions> for emlite::Val {
    fn from(s: WriterCreateCoreOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WriterCreateCoreOptions> for emlite::Val {
    fn from(s: &WriterCreateCoreOptions) -> emlite::Val {
        s.inner.clone()
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
    pub fn expected_input_languages(&self) -> Sequence<DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<Sequence<DOMString>>()
    }

    pub fn set_expected_input_languages(&mut self, value: Sequence<DOMString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn expected_context_languages(&self) -> Sequence<DOMString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<Sequence<DOMString>>()
    }

    pub fn set_expected_context_languages(&mut self, value: Sequence<DOMString>) {
        self.inner.set("expectedContextLanguages", value);
    }
}
impl WriterCreateCoreOptions {
    pub fn output_language(&self) -> DOMString {
        self.inner.get("outputLanguage").as_::<DOMString>()
    }

    pub fn set_output_language(&mut self, value: DOMString) {
        self.inner.set("outputLanguage", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for WriterWriteOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WriterWriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WriterWriteOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WriterWriteOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WriterWriteOptions> for emlite::Val {
    fn from(s: WriterWriteOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WriterWriteOptions> for emlite::Val {
    fn from(s: &WriterWriteOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl WriterWriteOptions {
    pub fn context(&self) -> DOMString {
        self.inner.get("context").as_::<DOMString>()
    }

    pub fn set_context(&mut self, value: DOMString) {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for Writer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Writer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Writer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Writer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Writer> for emlite::Val {
    fn from(s: Writer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Writer> for emlite::Val {
    fn from(s: &Writer) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Writer);

impl Writer {
    pub fn create0() -> Promise {
        emlite::Val::global("Writer")
            .call("create", &[])
            .as_::<Promise>()
    }

    pub fn create1(options: WriterCreateOptions) -> Promise {
        emlite::Val::global("Writer")
            .call("create", &[options.into()])
            .as_::<Promise>()
    }
}
impl Writer {
    pub fn availability0() -> Promise {
        emlite::Val::global("Writer")
            .call("availability", &[])
            .as_::<Promise>()
    }

    pub fn availability1(options: WriterCreateCoreOptions) -> Promise {
        emlite::Val::global("Writer")
            .call("availability", &[options.into()])
            .as_::<Promise>()
    }
}
impl Writer {
    pub fn write0(&self, input: DOMString) -> Promise {
        self.inner.call("write", &[input.into()]).as_::<Promise>()
    }

    pub fn write1(&self, input: DOMString, options: WriterWriteOptions) -> Promise {
        self.inner
            .call("write", &[input.into(), options.into()])
            .as_::<Promise>()
    }
}
impl Writer {
    pub fn write_streaming0(&self, input: DOMString) -> ReadableStream {
        self.inner
            .call("writeStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }

    pub fn write_streaming1(
        &self,
        input: DOMString,
        options: WriterWriteOptions,
    ) -> ReadableStream {
        self.inner
            .call("writeStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Writer {
    pub fn shared_context(&self) -> DOMString {
        self.inner.get("sharedContext").as_::<DOMString>()
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
    pub fn expected_input_languages(&self) -> FrozenArray<DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<FrozenArray<DOMString>>()
    }
}
impl Writer {
    pub fn expected_context_languages(&self) -> FrozenArray<DOMString> {
        self.inner
            .get("expectedContextLanguages")
            .as_::<FrozenArray<DOMString>>()
    }
}
impl Writer {
    pub fn output_language(&self) -> DOMString {
        self.inner.get("outputLanguage").as_::<DOMString>()
    }
}
impl Writer {
    pub fn measure_input_usage0(&self, input: DOMString) -> Promise {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise>()
    }

    pub fn measure_input_usage1(&self, input: DOMString, options: WriterWriteOptions) -> Promise {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise>()
    }
}
impl Writer {
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Writer {
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
