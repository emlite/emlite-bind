use super::*;

/// The SpeechRecognition class.
/// [`SpeechRecognition`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognition {
    inner: EventTarget,
}

impl FromVal for SpeechRecognition {
    fn from_val(v: &Any) -> Self {
        SpeechRecognition {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SpeechRecognition {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpeechRecognition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpeechRecognition {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechRecognition {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SpeechRecognition> for Any {
    fn from(s: SpeechRecognition) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechRecognition> for Any {
    fn from(s: &SpeechRecognition) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechRecognition);

impl SpeechRecognition {
    /// Getter of the `grammars` attribute.
    /// [`SpeechRecognition.grammars`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)
    pub fn grammars(&self) -> SpeechGrammarList {
        self.inner.get("grammars").as_::<SpeechGrammarList>()
    }

    /// Setter of the `grammars` attribute.
    /// [`SpeechRecognition.grammars`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/grammars)
    pub fn set_grammars(&mut self, value: &SpeechGrammarList) {
        self.inner.set("grammars", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `lang` attribute.
    /// [`SpeechRecognition.lang`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)
    pub fn lang(&self) -> JsString {
        self.inner.get("lang").as_::<JsString>()
    }

    /// Setter of the `lang` attribute.
    /// [`SpeechRecognition.lang`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/lang)
    pub fn set_lang(&mut self, value: &JsString) {
        self.inner.set("lang", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `continuous` attribute.
    /// [`SpeechRecognition.continuous`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)
    pub fn continuous(&self) -> bool {
        self.inner.get("continuous").as_::<bool>()
    }

    /// Setter of the `continuous` attribute.
    /// [`SpeechRecognition.continuous`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/continuous)
    pub fn set_continuous(&mut self, value: bool) {
        self.inner.set("continuous", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `interimResults` attribute.
    /// [`SpeechRecognition.interimResults`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)
    pub fn interim_results(&self) -> bool {
        self.inner.get("interimResults").as_::<bool>()
    }

    /// Setter of the `interimResults` attribute.
    /// [`SpeechRecognition.interimResults`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/interimResults)
    pub fn set_interim_results(&mut self, value: bool) {
        self.inner.set("interimResults", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `maxAlternatives` attribute.
    /// [`SpeechRecognition.maxAlternatives`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)
    pub fn max_alternatives(&self) -> u32 {
        self.inner.get("maxAlternatives").as_::<u32>()
    }

    /// Setter of the `maxAlternatives` attribute.
    /// [`SpeechRecognition.maxAlternatives`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/maxAlternatives)
    pub fn set_max_alternatives(&mut self, value: u32) {
        self.inner.set("maxAlternatives", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `processLocally` attribute.
    /// [`SpeechRecognition.processLocally`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/processLocally)
    pub fn process_locally(&self) -> bool {
        self.inner.get("processLocally").as_::<bool>()
    }

    /// Setter of the `processLocally` attribute.
    /// [`SpeechRecognition.processLocally`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/processLocally)
    pub fn set_process_locally(&mut self, value: bool) {
        self.inner.set("processLocally", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `phrases` attribute.
    /// [`SpeechRecognition.phrases`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/phrases)
    pub fn phrases(&self) -> TypedArray<SpeechRecognitionPhrase> {
        self.inner
            .get("phrases")
            .as_::<TypedArray<SpeechRecognitionPhrase>>()
    }

    /// Setter of the `phrases` attribute.
    /// [`SpeechRecognition.phrases`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/phrases)
    pub fn set_phrases(&mut self, value: &TypedArray<SpeechRecognitionPhrase>) {
        self.inner.set("phrases", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onaudiostart` attribute.
    /// [`SpeechRecognition.onaudiostart`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)
    pub fn onaudiostart(&self) -> Any {
        self.inner.get("onaudiostart").as_::<Any>()
    }

    /// Setter of the `onaudiostart` attribute.
    /// [`SpeechRecognition.onaudiostart`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudiostart)
    pub fn set_onaudiostart(&mut self, value: &Any) {
        self.inner.set("onaudiostart", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onsoundstart` attribute.
    /// [`SpeechRecognition.onsoundstart`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)
    pub fn onsoundstart(&self) -> Any {
        self.inner.get("onsoundstart").as_::<Any>()
    }

    /// Setter of the `onsoundstart` attribute.
    /// [`SpeechRecognition.onsoundstart`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundstart)
    pub fn set_onsoundstart(&mut self, value: &Any) {
        self.inner.set("onsoundstart", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onspeechstart` attribute.
    /// [`SpeechRecognition.onspeechstart`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)
    pub fn onspeechstart(&self) -> Any {
        self.inner.get("onspeechstart").as_::<Any>()
    }

    /// Setter of the `onspeechstart` attribute.
    /// [`SpeechRecognition.onspeechstart`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechstart)
    pub fn set_onspeechstart(&mut self, value: &Any) {
        self.inner.set("onspeechstart", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onspeechend` attribute.
    /// [`SpeechRecognition.onspeechend`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)
    pub fn onspeechend(&self) -> Any {
        self.inner.get("onspeechend").as_::<Any>()
    }

    /// Setter of the `onspeechend` attribute.
    /// [`SpeechRecognition.onspeechend`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onspeechend)
    pub fn set_onspeechend(&mut self, value: &Any) {
        self.inner.set("onspeechend", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onsoundend` attribute.
    /// [`SpeechRecognition.onsoundend`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)
    pub fn onsoundend(&self) -> Any {
        self.inner.get("onsoundend").as_::<Any>()
    }

    /// Setter of the `onsoundend` attribute.
    /// [`SpeechRecognition.onsoundend`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onsoundend)
    pub fn set_onsoundend(&mut self, value: &Any) {
        self.inner.set("onsoundend", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onaudioend` attribute.
    /// [`SpeechRecognition.onaudioend`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)
    pub fn onaudioend(&self) -> Any {
        self.inner.get("onaudioend").as_::<Any>()
    }

    /// Setter of the `onaudioend` attribute.
    /// [`SpeechRecognition.onaudioend`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onaudioend)
    pub fn set_onaudioend(&mut self, value: &Any) {
        self.inner.set("onaudioend", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onresult` attribute.
    /// [`SpeechRecognition.onresult`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)
    pub fn onresult(&self) -> Any {
        self.inner.get("onresult").as_::<Any>()
    }

    /// Setter of the `onresult` attribute.
    /// [`SpeechRecognition.onresult`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onresult)
    pub fn set_onresult(&mut self, value: &Any) {
        self.inner.set("onresult", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onnomatch` attribute.
    /// [`SpeechRecognition.onnomatch`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)
    pub fn onnomatch(&self) -> Any {
        self.inner.get("onnomatch").as_::<Any>()
    }

    /// Setter of the `onnomatch` attribute.
    /// [`SpeechRecognition.onnomatch`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onnomatch)
    pub fn set_onnomatch(&mut self, value: &Any) {
        self.inner.set("onnomatch", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onerror` attribute.
    /// [`SpeechRecognition.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`SpeechRecognition.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onstart` attribute.
    /// [`SpeechRecognition.onstart`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)
    pub fn onstart(&self) -> Any {
        self.inner.get("onstart").as_::<Any>()
    }

    /// Setter of the `onstart` attribute.
    /// [`SpeechRecognition.onstart`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onstart)
    pub fn set_onstart(&mut self, value: &Any) {
        self.inner.set("onstart", value);
    }
}
impl SpeechRecognition {
    /// Getter of the `onend` attribute.
    /// [`SpeechRecognition.onend`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)
    pub fn onend(&self) -> Any {
        self.inner.get("onend").as_::<Any>()
    }

    /// Setter of the `onend` attribute.
    /// [`SpeechRecognition.onend`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/onend)
    pub fn set_onend(&mut self, value: &Any) {
        self.inner.set("onend", value);
    }
}

impl SpeechRecognition {
    /// The `new SpeechRecognition(..)` constructor, creating a new SpeechRecognition instance
    pub fn new() -> SpeechRecognition {
        Self {
            inner: Any::global("SpeechRecognition")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }
}

impl SpeechRecognition {
    /// The start method.
    /// [`SpeechRecognition.start`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)
    pub fn start(&self) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }
}
impl SpeechRecognition {
    /// The start method.
    /// [`SpeechRecognition.start`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/start)
    pub fn start_with_audio_track(&self, audio_track: &MediaStreamTrack) -> Undefined {
        self.inner
            .call("start", &[audio_track.into()])
            .as_::<Undefined>()
    }
}
impl SpeechRecognition {
    /// The stop method.
    /// [`SpeechRecognition.stop`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/stop)
    pub fn stop(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
impl SpeechRecognition {
    /// The abort method.
    /// [`SpeechRecognition.abort`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/abort)
    pub fn abort(&self) -> Undefined {
        self.inner.call("abort", &[]).as_::<Undefined>()
    }
}
impl SpeechRecognition {
    /// The available method.
    /// [`SpeechRecognition.available`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/available)
    pub fn available(options: &SpeechRecognitionOptions) -> Promise<AvailabilityStatus> {
        Any::global("SpeechRecognition")
            .call("available", &[options.into()])
            .as_::<Promise<AvailabilityStatus>>()
    }
}
impl SpeechRecognition {
    /// The install method.
    /// [`SpeechRecognition.install`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognition/install)
    pub fn install(options: &SpeechRecognitionOptions) -> Promise<bool> {
        Any::global("SpeechRecognition")
            .call("install", &[options.into()])
            .as_::<Promise<bool>>()
    }
}
