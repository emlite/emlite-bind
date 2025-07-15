use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionOptions {
    inner: emlite::Val,
}
impl FromVal for SpeechRecognitionOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechRecognitionOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SpeechRecognitionOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechRecognitionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SpeechRecognitionOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SpeechRecognitionOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SpeechRecognitionOptions> for emlite::Val {
    fn from(s: SpeechRecognitionOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SpeechRecognitionOptions> for emlite::Val {
    fn from(s: &SpeechRecognitionOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl SpeechRecognitionOptions {
    pub fn langs(&self) -> Sequence<DOMString> {
        self.inner.get("langs").as_::<Sequence<DOMString>>()
    }

    pub fn set_langs(&mut self, value: Sequence<DOMString>) {
        self.inner.set("langs", value);
    }
}
impl SpeechRecognitionOptions {
    pub fn process_locally(&self) -> bool {
        self.inner.get("processLocally").as_::<bool>()
    }

    pub fn set_process_locally(&mut self, value: bool) {
        self.inner.set("processLocally", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognition {
    inner: EventTarget,
}
impl FromVal for SpeechRecognition {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechRecognition {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for SpeechRecognition {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SpeechRecognition {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SpeechRecognition> for emlite::Val {
    fn from(s: SpeechRecognition) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SpeechRecognition> for emlite::Val {
    fn from(s: &SpeechRecognition) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SpeechRecognition);

impl SpeechRecognition {
    pub fn new() -> SpeechRecognition {
        Self {
            inner: emlite::Val::global("SpeechRecognition")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }
}
impl SpeechRecognition {
    pub fn grammars(&self) -> SpeechGrammarList {
        self.inner.get("grammars").as_::<SpeechGrammarList>()
    }

    pub fn set_grammars(&mut self, value: SpeechGrammarList) {
        self.inner.set("grammars", value);
    }
}
impl SpeechRecognition {
    pub fn lang(&self) -> DOMString {
        self.inner.get("lang").as_::<DOMString>()
    }

    pub fn set_lang(&mut self, value: DOMString) {
        self.inner.set("lang", value);
    }
}
impl SpeechRecognition {
    pub fn continuous(&self) -> bool {
        self.inner.get("continuous").as_::<bool>()
    }

    pub fn set_continuous(&mut self, value: bool) {
        self.inner.set("continuous", value);
    }
}
impl SpeechRecognition {
    pub fn interim_results(&self) -> bool {
        self.inner.get("interimResults").as_::<bool>()
    }

    pub fn set_interim_results(&mut self, value: bool) {
        self.inner.set("interimResults", value);
    }
}
impl SpeechRecognition {
    pub fn max_alternatives(&self) -> u32 {
        self.inner.get("maxAlternatives").as_::<u32>()
    }

    pub fn set_max_alternatives(&mut self, value: u32) {
        self.inner.set("maxAlternatives", value);
    }
}
impl SpeechRecognition {
    pub fn process_locally(&self) -> bool {
        self.inner.get("processLocally").as_::<bool>()
    }

    pub fn set_process_locally(&mut self, value: bool) {
        self.inner.set("processLocally", value);
    }
}
impl SpeechRecognition {
    pub fn phrases(&self) -> ObservableArray<SpeechRecognitionPhrase> {
        self.inner
            .get("phrases")
            .as_::<ObservableArray<SpeechRecognitionPhrase>>()
    }

    pub fn set_phrases(&mut self, value: ObservableArray<SpeechRecognitionPhrase>) {
        self.inner.set("phrases", value);
    }
}
impl SpeechRecognition {
    pub fn start(&self, audio_track: MediaStreamTrack) -> Undefined {
        self.inner
            .call("start", &[audio_track.into()])
            .as_::<Undefined>()
    }
}
impl SpeechRecognition {
    pub fn stop(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
impl SpeechRecognition {
    pub fn abort(&self) -> Undefined {
        self.inner.call("abort", &[]).as_::<Undefined>()
    }
}
impl SpeechRecognition {
    pub fn available(options: SpeechRecognitionOptions) -> Promise {
        emlite::Val::global("SpeechRecognition")
            .call("available", &[options.into()])
            .as_::<Promise>()
    }
}
impl SpeechRecognition {
    pub fn install(options: SpeechRecognitionOptions) -> Promise {
        emlite::Val::global("SpeechRecognition")
            .call("install", &[options.into()])
            .as_::<Promise>()
    }
}
impl SpeechRecognition {
    pub fn onaudiostart(&self) -> Any {
        self.inner.get("onaudiostart").as_::<Any>()
    }

    pub fn set_onaudiostart(&mut self, value: Any) {
        self.inner.set("onaudiostart", value);
    }
}
impl SpeechRecognition {
    pub fn onsoundstart(&self) -> Any {
        self.inner.get("onsoundstart").as_::<Any>()
    }

    pub fn set_onsoundstart(&mut self, value: Any) {
        self.inner.set("onsoundstart", value);
    }
}
impl SpeechRecognition {
    pub fn onspeechstart(&self) -> Any {
        self.inner.get("onspeechstart").as_::<Any>()
    }

    pub fn set_onspeechstart(&mut self, value: Any) {
        self.inner.set("onspeechstart", value);
    }
}
impl SpeechRecognition {
    pub fn onspeechend(&self) -> Any {
        self.inner.get("onspeechend").as_::<Any>()
    }

    pub fn set_onspeechend(&mut self, value: Any) {
        self.inner.set("onspeechend", value);
    }
}
impl SpeechRecognition {
    pub fn onsoundend(&self) -> Any {
        self.inner.get("onsoundend").as_::<Any>()
    }

    pub fn set_onsoundend(&mut self, value: Any) {
        self.inner.set("onsoundend", value);
    }
}
impl SpeechRecognition {
    pub fn onaudioend(&self) -> Any {
        self.inner.get("onaudioend").as_::<Any>()
    }

    pub fn set_onaudioend(&mut self, value: Any) {
        self.inner.set("onaudioend", value);
    }
}
impl SpeechRecognition {
    pub fn onresult(&self) -> Any {
        self.inner.get("onresult").as_::<Any>()
    }

    pub fn set_onresult(&mut self, value: Any) {
        self.inner.set("onresult", value);
    }
}
impl SpeechRecognition {
    pub fn onnomatch(&self) -> Any {
        self.inner.get("onnomatch").as_::<Any>()
    }

    pub fn set_onnomatch(&mut self, value: Any) {
        self.inner.set("onnomatch", value);
    }
}
impl SpeechRecognition {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: Any) {
        self.inner.set("onerror", value);
    }
}
impl SpeechRecognition {
    pub fn onstart(&self) -> Any {
        self.inner.get("onstart").as_::<Any>()
    }

    pub fn set_onstart(&mut self, value: Any) {
        self.inner.set("onstart", value);
    }
}
impl SpeechRecognition {
    pub fn onend(&self) -> Any {
        self.inner.get("onend").as_::<Any>()
    }

    pub fn set_onend(&mut self, value: Any) {
        self.inner.set("onend", value);
    }
}
