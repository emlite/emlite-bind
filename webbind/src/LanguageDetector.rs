use super::*;

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
    /// Getter of the `expectedInputLanguages` attribute.
    /// [`LanguageDetector.expectedInputLanguages`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/expectedInputLanguages)
    pub fn expected_input_languages(&self) -> TypedArray<JsString> {
        self.inner
            .get("expectedInputLanguages")
            .as_::<TypedArray<JsString>>()
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
    /// The create method.
    /// [`LanguageDetector.create`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/create)
    pub fn create() -> Promise<LanguageDetector> {
        Any::global("LanguageDetector")
            .call("create", &[])
            .as_::<Promise<LanguageDetector>>()
    }
}
impl LanguageDetector {
    /// The create method.
    /// [`LanguageDetector.create`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/create)
    pub fn create_with_options(
        options: &LanguageDetectorCreateOptions,
    ) -> Promise<LanguageDetector> {
        Any::global("LanguageDetector")
            .call("create", &[options.into()])
            .as_::<Promise<LanguageDetector>>()
    }
}
impl LanguageDetector {
    /// The availability method.
    /// [`LanguageDetector.availability`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/availability)
    pub fn availability() -> Promise<Availability> {
        Any::global("LanguageDetector")
            .call("availability", &[])
            .as_::<Promise<Availability>>()
    }
}
impl LanguageDetector {
    /// The availability method.
    /// [`LanguageDetector.availability`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/availability)
    pub fn availability_with_options(
        options: &LanguageDetectorCreateCoreOptions,
    ) -> Promise<Availability> {
        Any::global("LanguageDetector")
            .call("availability", &[options.into()])
            .as_::<Promise<Availability>>()
    }
}
impl LanguageDetector {
    /// The detect method.
    /// [`LanguageDetector.detect`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/detect)
    pub fn detect(&self, input: &JsString) -> Promise<TypedArray<LanguageDetectionResult>> {
        self.inner
            .call("detect", &[input.into()])
            .as_::<Promise<TypedArray<LanguageDetectionResult>>>()
    }
}
impl LanguageDetector {
    /// The detect method.
    /// [`LanguageDetector.detect`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/detect)
    pub fn detect_with_options(
        &self,
        input: &JsString,
        options: &LanguageDetectorDetectOptions,
    ) -> Promise<TypedArray<LanguageDetectionResult>> {
        self.inner
            .call("detect", &[input.into(), options.into()])
            .as_::<Promise<TypedArray<LanguageDetectionResult>>>()
    }
}
impl LanguageDetector {
    /// The measureInputUsage method.
    /// [`LanguageDetector.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/measureInputUsage)
    pub fn measure_input_usage(&self, input: &JsString) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into()])
            .as_::<Promise<f64>>()
    }
}
impl LanguageDetector {
    /// The measureInputUsage method.
    /// [`LanguageDetector.measureInputUsage`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/measureInputUsage)
    pub fn measure_input_usage_with_options(
        &self,
        input: &JsString,
        options: &LanguageDetectorDetectOptions,
    ) -> Promise<f64> {
        self.inner
            .call("measureInputUsage", &[input.into(), options.into()])
            .as_::<Promise<f64>>()
    }
}
impl LanguageDetector {
    /// The destroy method.
    /// [`LanguageDetector.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/LanguageDetector/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
