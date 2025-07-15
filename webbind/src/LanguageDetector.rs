use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for LanguageDetectorCreateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LanguageDetectorCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LanguageDetectorCreateOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LanguageDetectorCreateOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LanguageDetectorCreateOptions> for emlite::Val {
    fn from(s: LanguageDetectorCreateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&LanguageDetectorCreateOptions> for emlite::Val {
    fn from(s: &LanguageDetectorCreateOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl LanguageDetectorCreateOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl LanguageDetectorCreateOptions {
    pub fn monitor(&self) -> Function {
        self.inner.get("monitor").as_::<Function>()
    }

    pub fn set_monitor(&mut self, value: &Function) {
        self.inner.set("monitor", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for LanguageDetectorCreateCoreOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LanguageDetectorCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LanguageDetectorCreateCoreOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LanguageDetectorCreateCoreOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LanguageDetectorCreateCoreOptions> for emlite::Val {
    fn from(s: LanguageDetectorCreateCoreOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&LanguageDetectorCreateCoreOptions> for emlite::Val {
    fn from(s: &LanguageDetectorCreateCoreOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl LanguageDetectorCreateCoreOptions {
    pub fn expected_input_languages(&self) -> Sequence<String> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<Sequence<String>>()
    }

    pub fn set_expected_input_languages(&mut self, value: &Sequence<String>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for LanguageDetectionResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LanguageDetectionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LanguageDetectionResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LanguageDetectionResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LanguageDetectionResult> for emlite::Val {
    fn from(s: LanguageDetectionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&LanguageDetectionResult> for emlite::Val {
    fn from(s: &LanguageDetectionResult) -> emlite::Val {
        s.inner.clone()
    }
}

impl LanguageDetectionResult {
    pub fn detected_language(&self) -> String {
        self.inner.get("detectedLanguage").as_::<String>()
    }

    pub fn set_detected_language(&mut self, value: &str) {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for LanguageDetectorDetectOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LanguageDetectorDetectOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LanguageDetectorDetectOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LanguageDetectorDetectOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LanguageDetectorDetectOptions> for emlite::Val {
    fn from(s: LanguageDetectorDetectOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&LanguageDetectorDetectOptions> for emlite::Val {
    fn from(s: &LanguageDetectorDetectOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl LanguageDetectorDetectOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for LanguageDetector {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LanguageDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for LanguageDetector {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LanguageDetector {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<LanguageDetector> for emlite::Val {
    fn from(s: LanguageDetector) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&LanguageDetector> for emlite::Val {
    fn from(s: &LanguageDetector) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LanguageDetector);

impl LanguageDetector {
    pub fn create0() -> Promise {
        emlite::Val::global("LanguageDetector")
            .call("create", &[])
            .as_::<Promise>()
    }

    pub fn create1(options: &LanguageDetectorCreateOptions) -> Promise {
        emlite::Val::global("LanguageDetector")
            .call("create", &[options.into()])
            .as_::<Promise>()
    }
}
impl LanguageDetector {
    pub fn availability0() -> Promise {
        emlite::Val::global("LanguageDetector")
            .call("availability", &[])
            .as_::<Promise>()
    }

    pub fn availability1(options: &LanguageDetectorCreateCoreOptions) -> Promise {
        emlite::Val::global("LanguageDetector")
            .call("availability", &[options.into()])
            .as_::<Promise>()
    }
}
impl LanguageDetector {
    pub fn detect0(&self, input: &str) -> Promise {
        self.inner.call("detect", &[input.into()]).as_::<Promise>()
    }

    pub fn detect1(&self, input: &str, options: &LanguageDetectorDetectOptions) -> Promise {
        self.inner
            .call("detect", &[input.into(), options.into()])
            .as_::<Promise>()
    }
}
impl LanguageDetector {
    pub fn expected_input_languages(&self) -> FrozenArray<String> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<FrozenArray<String>>()
    }
}
impl LanguageDetector {
    pub fn measure_input_usage0(&self, input: &str) -> Promise {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise>()
    }

    pub fn measure_input_usage1(
        &self,
        input: &str,
        options: &LanguageDetectorDetectOptions,
    ) -> Promise {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise>()
    }
}
impl LanguageDetector {
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl LanguageDetector {
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
