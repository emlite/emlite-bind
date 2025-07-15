use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for TranslatorCreateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TranslatorCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TranslatorCreateOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TranslatorCreateOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TranslatorCreateOptions> for emlite::Val {
    fn from(s: TranslatorCreateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&TranslatorCreateOptions> for emlite::Val {
    fn from(s: &TranslatorCreateOptions) -> emlite::Val {
        s.inner.clone()
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
    pub fn monitor(&self) -> Function {
        self.inner.get("monitor").as_::<Function>()
    }

    pub fn set_monitor(&mut self, value: Function) {
        self.inner.set("monitor", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for TranslatorCreateCoreOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TranslatorCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TranslatorCreateCoreOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TranslatorCreateCoreOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TranslatorCreateCoreOptions> for emlite::Val {
    fn from(s: TranslatorCreateCoreOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&TranslatorCreateCoreOptions> for emlite::Val {
    fn from(s: &TranslatorCreateCoreOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl TranslatorCreateCoreOptions {
    pub fn source_language(&self) -> DOMString {
        self.inner.get("sourceLanguage").as_::<DOMString>()
    }

    pub fn set_source_language(&mut self, value: DOMString) {
        self.inner.set("sourceLanguage", value);
    }
}
impl TranslatorCreateCoreOptions {
    pub fn target_language(&self) -> DOMString {
        self.inner.get("targetLanguage").as_::<DOMString>()
    }

    pub fn set_target_language(&mut self, value: DOMString) {
        self.inner.set("targetLanguage", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for TranslatorTranslateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TranslatorTranslateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TranslatorTranslateOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TranslatorTranslateOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TranslatorTranslateOptions> for emlite::Val {
    fn from(s: TranslatorTranslateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&TranslatorTranslateOptions> for emlite::Val {
    fn from(s: &TranslatorTranslateOptions) -> emlite::Val {
        s.inner.clone()
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for Translator {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Translator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Translator {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Translator {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Translator> for emlite::Val {
    fn from(s: Translator) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Translator> for emlite::Val {
    fn from(s: &Translator) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Translator);

impl Translator {
    pub fn create(options: TranslatorCreateOptions) -> Promise {
        emlite::Val::global("Translator")
            .call("create", &[options.into()])
            .as_::<Promise>()
    }
}
impl Translator {
    pub fn availability(options: TranslatorCreateCoreOptions) -> Promise {
        emlite::Val::global("Translator")
            .call("availability", &[options.into()])
            .as_::<Promise>()
    }
}
impl Translator {
    pub fn translate0(&self, input: DOMString) -> Promise {
        self.inner
            .call("translate", &[input.into()])
            .as_::<Promise>()
    }

    pub fn translate1(&self, input: DOMString, options: TranslatorTranslateOptions) -> Promise {
        self.inner
            .call("translate", &[input.into(), options.into()])
            .as_::<Promise>()
    }
}
impl Translator {
    pub fn translate_streaming0(&self, input: DOMString) -> ReadableStream {
        self.inner
            .call("translateStreaming", &[input.into()])
            .as_::<ReadableStream>()
    }

    pub fn translate_streaming1(
        &self,
        input: DOMString,
        options: TranslatorTranslateOptions,
    ) -> ReadableStream {
        self.inner
            .call("translateStreaming", &[input.into(), options.into()])
            .as_::<ReadableStream>()
    }
}
impl Translator {
    pub fn source_language(&self) -> DOMString {
        self.inner.get("sourceLanguage").as_::<DOMString>()
    }
}
impl Translator {
    pub fn target_language(&self) -> DOMString {
        self.inner.get("targetLanguage").as_::<DOMString>()
    }
}
impl Translator {
    pub fn measure_input_usage0(&self, input: DOMString) -> Promise {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise>()
    }

    pub fn measure_input_usage1(
        &self,
        input: DOMString,
        options: TranslatorTranslateOptions,
    ) -> Promise {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise>()
    }
}
impl Translator {
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl Translator {
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
