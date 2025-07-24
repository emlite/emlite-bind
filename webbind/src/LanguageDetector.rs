use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LanguageDetectorCreateOptions {
    inner: Any,
}
impl FromVal for LanguageDetectorCreateOptions {
    fn from_val(v: &Any) -> Self {
        LanguageDetectorCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LanguageDetectorCreateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LanguageDetectorCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LanguageDetectorCreateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LanguageDetectorCreateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LanguageDetectorCreateOptions> for Any {
    fn from(s: LanguageDetectorCreateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LanguageDetectorCreateOptions> for Any {
    fn from(s: &LanguageDetectorCreateOptions) -> Any {
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
    inner: Any,
}
impl FromVal for LanguageDetectorCreateCoreOptions {
    fn from_val(v: &Any) -> Self {
        LanguageDetectorCreateCoreOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LanguageDetectorCreateCoreOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LanguageDetectorCreateCoreOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LanguageDetectorCreateCoreOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LanguageDetectorCreateCoreOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LanguageDetectorCreateCoreOptions> for Any {
    fn from(s: LanguageDetectorCreateCoreOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LanguageDetectorCreateCoreOptions> for Any {
    fn from(s: &LanguageDetectorCreateCoreOptions) -> Any {
        s.inner.clone()
    }
}

impl LanguageDetectorCreateCoreOptions {
    pub fn expected_input_languages(&self) -> Sequence<DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<Sequence<DOMString>>()
    }

    pub fn set_expected_input_languages(&mut self, value: &Sequence<DOMString>) {
        self.inner.set("expectedInputLanguages", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LanguageDetectionResult {
    inner: Any,
}
impl FromVal for LanguageDetectionResult {
    fn from_val(v: &Any) -> Self {
        LanguageDetectionResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LanguageDetectionResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LanguageDetectionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LanguageDetectionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LanguageDetectionResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LanguageDetectionResult> for Any {
    fn from(s: LanguageDetectionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LanguageDetectionResult> for Any {
    fn from(s: &LanguageDetectionResult) -> Any {
        s.inner.clone()
    }
}

impl LanguageDetectionResult {
    pub fn detected_language(&self) -> DOMString {
        self.inner.get("detectedLanguage").as_::<DOMString>()
    }

    pub fn set_detected_language(&mut self, value: &DOMString) {
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
    inner: Any,
}
impl FromVal for LanguageDetectorDetectOptions {
    fn from_val(v: &Any) -> Self {
        LanguageDetectorDetectOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for LanguageDetectorDetectOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LanguageDetectorDetectOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LanguageDetectorDetectOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LanguageDetectorDetectOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LanguageDetectorDetectOptions> for Any {
    fn from(s: LanguageDetectorDetectOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LanguageDetectorDetectOptions> for Any {
    fn from(s: &LanguageDetectorDetectOptions) -> Any {
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
/// The LanguageDetector class.
/// [`LanguageDetector`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LanguageDetector {
    inner: Any,
}
impl FromVal for LanguageDetector {
    fn from_val(v: &Any) -> Self {
        LanguageDetector {
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
impl core::ops::Deref for LanguageDetector {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LanguageDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LanguageDetector {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LanguageDetector {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LanguageDetector> for Any {
    fn from(s: LanguageDetector) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LanguageDetector> for Any {
    fn from(s: &LanguageDetector) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LanguageDetector);

impl LanguageDetector {
    /// The create method.
    /// [`LanguageDetector.create`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/create)
    pub fn create0() -> Promise<LanguageDetector> {
        Any::global("LanguageDetector")
            .call("create", &[])
            .as_::<Promise<LanguageDetector>>()
    }
    /// The create method.
    /// [`LanguageDetector.create`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/create)
    pub fn create1(options: &LanguageDetectorCreateOptions) -> Promise<LanguageDetector> {
        Any::global("LanguageDetector")
            .call("create", &[options.into()])
            .as_::<Promise<LanguageDetector>>()
    }
}
impl LanguageDetector {
    /// The availability method.
    /// [`LanguageDetector.availability`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/availability)
    pub fn availability0() -> Promise<Availability> {
        Any::global("LanguageDetector")
            .call("availability", &[])
            .as_::<Promise<Availability>>()
    }
    /// The availability method.
    /// [`LanguageDetector.availability`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/availability)
    pub fn availability1(options: &LanguageDetectorCreateCoreOptions) -> Promise<Availability> {
        Any::global("LanguageDetector")
            .call("availability", &[options.into()])
            .as_::<Promise<Availability>>()
    }
}
impl LanguageDetector {
    /// The detect method.
    /// [`LanguageDetector.detect`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/detect)
    pub fn detect0(&self, input: &DOMString) -> Promise<Sequence<LanguageDetectionResult>> {
        self.inner
            .call("detect", &[input.into()])
            .as_::<Promise<Sequence<LanguageDetectionResult>>>()
    }
    /// The detect method.
    /// [`LanguageDetector.detect`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/detect)
    pub fn detect1(
        &self,
        input: &DOMString,
        options: &LanguageDetectorDetectOptions,
    ) -> Promise<Sequence<LanguageDetectionResult>> {
        self.inner
            .call("detect", &[input.into(), options.into()])
            .as_::<Promise<Sequence<LanguageDetectionResult>>>()
    }
}
impl LanguageDetector {
    /// Getter of the `expectedInputLanguages` attribute.
    /// [`LanguageDetector.expectedInputLanguages`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/expectedInputLanguages)
    pub fn expected_input_languages(&self) -> FrozenArray<DOMString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<FrozenArray<DOMString>>()
    }
}
impl LanguageDetector {
    /// The measureInputUsage method.
    /// [`LanguageDetector.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/measureInputUsage)
    pub fn measure_input_usage0(&self, input: &DOMString) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise<f64>>()
    }
    /// The measureInputUsage method.
    /// [`LanguageDetector.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/measureInputUsage)
    pub fn measure_input_usage1(
        &self,
        input: &DOMString,
        options: &LanguageDetectorDetectOptions,
    ) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise<f64>>()
    }
}
impl LanguageDetector {
    /// Getter of the `inputQuota` attribute.
    /// [`LanguageDetector.inputQuota`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/inputQuota)
    pub fn input_quota(&self) -> f64 {
        self.inner.get("inputQuota").as_::<f64>()
    }
}
impl LanguageDetector {
    /// The destroy method.
    /// [`LanguageDetector.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
