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

impl SpeechRecognitionOptions {
    pub fn langs(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("langs")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_langs(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
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
    pub fn lang(&self) -> jsbind::DOMString {
        self.inner.get("lang").as_::<jsbind::DOMString>()
    }

    pub fn set_lang(&mut self, value: jsbind::DOMString) {
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
    pub fn phrases(&self) -> jsbind::ObservableArray<SpeechRecognitionPhrase> {
        self.inner
            .get("phrases")
            .as_::<jsbind::ObservableArray<SpeechRecognitionPhrase>>()
    }

    pub fn set_phrases(&mut self, value: jsbind::ObservableArray<SpeechRecognitionPhrase>) {
        self.inner.set("phrases", value);
    }
}
impl SpeechRecognition {
    pub fn start(&self, audio_track: MediaStreamTrack) -> jsbind::Undefined {
        self.inner
            .call("start", &[audio_track.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SpeechRecognition {
    pub fn stop(&self) -> jsbind::Undefined {
        self.inner.call("stop", &[]).as_::<jsbind::Undefined>()
    }
}
impl SpeechRecognition {
    pub fn abort(&self) -> jsbind::Undefined {
        self.inner.call("abort", &[]).as_::<jsbind::Undefined>()
    }
}
impl SpeechRecognition {
    pub fn available(options: SpeechRecognitionOptions) -> jsbind::Promise {
        emlite::Val::global("speechrecognition")
            .call("available", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SpeechRecognition {
    pub fn install(options: SpeechRecognitionOptions) -> jsbind::Promise {
        emlite::Val::global("speechrecognition")
            .call("install", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SpeechRecognition {
    pub fn onaudiostart(&self) -> jsbind::Any {
        self.inner.get("onaudiostart").as_::<jsbind::Any>()
    }

    pub fn set_onaudiostart(&mut self, value: jsbind::Any) {
        self.inner.set("onaudiostart", value);
    }
}
impl SpeechRecognition {
    pub fn onsoundstart(&self) -> jsbind::Any {
        self.inner.get("onsoundstart").as_::<jsbind::Any>()
    }

    pub fn set_onsoundstart(&mut self, value: jsbind::Any) {
        self.inner.set("onsoundstart", value);
    }
}
impl SpeechRecognition {
    pub fn onspeechstart(&self) -> jsbind::Any {
        self.inner.get("onspeechstart").as_::<jsbind::Any>()
    }

    pub fn set_onspeechstart(&mut self, value: jsbind::Any) {
        self.inner.set("onspeechstart", value);
    }
}
impl SpeechRecognition {
    pub fn onspeechend(&self) -> jsbind::Any {
        self.inner.get("onspeechend").as_::<jsbind::Any>()
    }

    pub fn set_onspeechend(&mut self, value: jsbind::Any) {
        self.inner.set("onspeechend", value);
    }
}
impl SpeechRecognition {
    pub fn onsoundend(&self) -> jsbind::Any {
        self.inner.get("onsoundend").as_::<jsbind::Any>()
    }

    pub fn set_onsoundend(&mut self, value: jsbind::Any) {
        self.inner.set("onsoundend", value);
    }
}
impl SpeechRecognition {
    pub fn onaudioend(&self) -> jsbind::Any {
        self.inner.get("onaudioend").as_::<jsbind::Any>()
    }

    pub fn set_onaudioend(&mut self, value: jsbind::Any) {
        self.inner.set("onaudioend", value);
    }
}
impl SpeechRecognition {
    pub fn onresult(&self) -> jsbind::Any {
        self.inner.get("onresult").as_::<jsbind::Any>()
    }

    pub fn set_onresult(&mut self, value: jsbind::Any) {
        self.inner.set("onresult", value);
    }
}
impl SpeechRecognition {
    pub fn onnomatch(&self) -> jsbind::Any {
        self.inner.get("onnomatch").as_::<jsbind::Any>()
    }

    pub fn set_onnomatch(&mut self, value: jsbind::Any) {
        self.inner.set("onnomatch", value);
    }
}
impl SpeechRecognition {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl SpeechRecognition {
    pub fn onstart(&self) -> jsbind::Any {
        self.inner.get("onstart").as_::<jsbind::Any>()
    }

    pub fn set_onstart(&mut self, value: jsbind::Any) {
        self.inner.set("onstart", value);
    }
}
impl SpeechRecognition {
    pub fn onend(&self) -> jsbind::Any {
        self.inner.get("onend").as_::<jsbind::Any>()
    }

    pub fn set_onend(&mut self, value: jsbind::Any) {
        self.inner.set("onend", value);
    }
}
