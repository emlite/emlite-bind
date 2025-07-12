use super::*;

#[derive(Clone, Debug)]
pub struct LanguageDetectorCreateOptions {
    inner: emlite::Val,
}
impl FromVal for LanguageDetectorCreateOptions {
    fn from_val(v: &emlite::Val) -> Self {
        LanguageDetectorCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LanguageDetectorCreateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LanguageDetectorCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LanguageDetectorCreateOptions> for emlite::Val {
    fn from(s: LanguageDetectorCreateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LanguageDetectorCreateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl LanguageDetectorCreateOptions {
    pub fn monitor(&self) -> jsbind::Function {
        self.inner.get("monitor").as_::<jsbind::Function>()
    }

    pub fn set_monitor(&mut self, value: jsbind::Function) {
        self.inner.set("monitor", value);
    }
}
#[derive(Clone, Debug)]
pub struct LanguageDetectorCreateCoreOptions {
    inner: emlite::Val,
}
impl FromVal for LanguageDetectorCreateCoreOptions {
    fn from_val(v: &emlite::Val) -> Self {
        LanguageDetectorCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LanguageDetectorCreateCoreOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LanguageDetectorCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LanguageDetectorCreateCoreOptions> for emlite::Val {
    fn from(s: LanguageDetectorCreateCoreOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LanguageDetectorCreateCoreOptions {
    pub fn expected_input_languages(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_expected_input_languages(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
#[derive(Clone, Debug)]
pub struct LanguageDetectionResult {
    inner: emlite::Val,
}
impl FromVal for LanguageDetectionResult {
    fn from_val(v: &emlite::Val) -> Self {
        LanguageDetectionResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LanguageDetectionResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LanguageDetectionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LanguageDetectionResult> for emlite::Val {
    fn from(s: LanguageDetectionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LanguageDetectionResult {
    pub fn detected_language(&self) -> jsbind::DOMString {
        self.inner
            .get("detectedLanguage")
            .as_::<jsbind::DOMString>()
    }

    pub fn set_detected_language(&mut self, value: jsbind::DOMString) {
        self.inner.set("detectedLanguage", value);
    }
}
impl LanguageDetectionResult {
    pub fn confidence(&self) -> f64 {
        self.inner.get("confidence").as_::<f64>()
    }

    pub fn set_confidence(&mut self, value: f64) {
        self.inner.set("confidence", value);
    }
}
#[derive(Clone, Debug)]
pub struct LanguageDetectorDetectOptions {
    inner: emlite::Val,
}
impl FromVal for LanguageDetectorDetectOptions {
    fn from_val(v: &emlite::Val) -> Self {
        LanguageDetectorDetectOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for LanguageDetectorDetectOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LanguageDetectorDetectOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LanguageDetectorDetectOptions> for emlite::Val {
    fn from(s: LanguageDetectorDetectOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LanguageDetectorDetectOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug)]
pub struct LanguageDetector {
    inner: emlite::Val,
}
impl FromVal for LanguageDetector {
    fn from_val(v: &emlite::Val) -> Self {
        LanguageDetector {
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
impl std::ops::Deref for LanguageDetector {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for LanguageDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LanguageDetector> for emlite::Val {
    fn from(s: LanguageDetector) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LanguageDetector {
    pub fn create0() -> jsbind::Promise {
        emlite::Val::global("languagedetector")
            .call("create", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn create1(options: LanguageDetectorCreateOptions) -> jsbind::Promise {
        emlite::Val::global("languagedetector")
            .call("create", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl LanguageDetector {
    pub fn availability0() -> jsbind::Promise {
        emlite::Val::global("languagedetector")
            .call("availability", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn availability1(options: LanguageDetectorCreateCoreOptions) -> jsbind::Promise {
        emlite::Val::global("languagedetector")
            .call("availability", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl LanguageDetector {
    pub fn detect0(&self, input: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("detect", &[input.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn detect1(
        &self,
        input: jsbind::DOMString,
        options: LanguageDetectorDetectOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("detect", &[input.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl LanguageDetector {
    pub fn expected_input_languages(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl LanguageDetector {
    pub fn measure_input_usage0(&self, input: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn measure_input_usage1(
        &self,
        input: jsbind::DOMString,
        options: LanguageDetectorDetectOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl LanguageDetector {
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl LanguageDetector {
    pub fn destroy(&self) -> jsbind::Undefined {
        self.inner.call("destroy", &[]).as_::<jsbind::Undefined>()
    }
}
