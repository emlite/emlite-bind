use super::*;

#[derive(Clone, Debug)]
pub struct TranslatorCreateOptions {
    inner: emlite::Val,
}
impl FromVal for TranslatorCreateOptions {
    fn from_val(v: &emlite::Val) -> Self {
        TranslatorCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TranslatorCreateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TranslatorCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TranslatorCreateOptions> for emlite::Val {
    fn from(s: TranslatorCreateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TranslatorCreateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl TranslatorCreateOptions {
    pub fn monitor(&self) -> jsbind::Function {
        self.inner.get("monitor").as_::<jsbind::Function>()
    }

    pub fn set_monitor(&mut self, value: jsbind::Function) {
        self.inner.set("monitor", value);
    }
}
#[derive(Clone, Debug)]
pub struct TranslatorCreateCoreOptions {
    inner: emlite::Val,
}
impl FromVal for TranslatorCreateCoreOptions {
    fn from_val(v: &emlite::Val) -> Self {
        TranslatorCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TranslatorCreateCoreOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TranslatorCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TranslatorCreateCoreOptions> for emlite::Val {
    fn from(s: TranslatorCreateCoreOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TranslatorCreateCoreOptions {
    pub fn source_language(&self) -> jsbind::DOMString {
        self.inner.get("sourceLanguage").as_::<jsbind::DOMString>()
    }

    pub fn set_source_language(&mut self, value: jsbind::DOMString) {
        self.inner.set("sourceLanguage", value);
    }
}
impl TranslatorCreateCoreOptions {
    pub fn target_language(&self) -> jsbind::DOMString {
        self.inner.get("targetLanguage").as_::<jsbind::DOMString>()
    }

    pub fn set_target_language(&mut self, value: jsbind::DOMString) {
        self.inner.set("targetLanguage", value);
    }
}
#[derive(Clone, Debug)]
pub struct TranslatorTranslateOptions {
    inner: emlite::Val,
}
impl FromVal for TranslatorTranslateOptions {
    fn from_val(v: &emlite::Val) -> Self {
        TranslatorTranslateOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for TranslatorTranslateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for TranslatorTranslateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TranslatorTranslateOptions> for emlite::Val {
    fn from(s: TranslatorTranslateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TranslatorTranslateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug)]
pub struct Translator {
    inner: emlite::Val,
}
impl FromVal for Translator {
    fn from_val(v: &emlite::Val) -> Self {
        Translator {
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
impl std::ops::Deref for Translator {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Translator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Translator> for emlite::Val {
    fn from(s: Translator) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Translator {
    pub fn create(options: TranslatorCreateOptions) -> jsbind::Promise {
        emlite::Val::global("translator")
            .call("create", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Translator {
    pub fn availability(options: TranslatorCreateCoreOptions) -> jsbind::Promise {
        emlite::Val::global("translator")
            .call("availability", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Translator {
    pub fn translate0(&self, input: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("translate", &[input.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn translate1(
        &self,
        input: jsbind::DOMString,
        options: TranslatorTranslateOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("translate", &[input.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Translator {
    pub fn translate_streaming0(&self, input: jsbind::DOMString) -> ReadableStream {
        self.inner
            .call("translateStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }

    pub fn translate_streaming1(
        &self,
        input: jsbind::DOMString,
        options: TranslatorTranslateOptions,
    ) -> ReadableStream {
        self.inner
            .call("translateStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Translator {
    pub fn source_language(&self) -> jsbind::DOMString {
        self.inner.get("sourceLanguage").as_::<jsbind::DOMString>()
    }
}
impl Translator {
    pub fn target_language(&self) -> jsbind::DOMString {
        self.inner.get("targetLanguage").as_::<jsbind::DOMString>()
    }
}
impl Translator {
    pub fn measure_input_usage0(&self, input: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn measure_input_usage1(
        &self,
        input: jsbind::DOMString,
        options: TranslatorTranslateOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Translator {
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Translator {
    pub fn destroy(&self) -> jsbind::Undefined {
        self.inner.call("destroy", &[]).as_::<jsbind::Undefined>()
    }
}
