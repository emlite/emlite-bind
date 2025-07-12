use super::*;

#[derive(Clone, Debug)]
pub struct SpeechSynthesisVoice {
    inner: emlite::Val,
}
impl FromVal for SpeechSynthesisVoice {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechSynthesisVoice {
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
impl std::ops::Deref for SpeechSynthesisVoice {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SpeechSynthesisVoice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechSynthesisVoice> for emlite::Val {
    fn from(s: SpeechSynthesisVoice) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechSynthesisVoice {
    pub fn voice_uri(&self) -> jsbind::DOMString {
        self.inner.get("voiceURI").as_::<jsbind::DOMString>()
    }
}
impl SpeechSynthesisVoice {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl SpeechSynthesisVoice {
    pub fn lang(&self) -> jsbind::DOMString {
        self.inner.get("lang").as_::<jsbind::DOMString>()
    }
}
impl SpeechSynthesisVoice {
    pub fn local_service(&self) -> bool {
        self.inner.get("localService").as_::<bool>()
    }
}
impl SpeechSynthesisVoice {
    pub fn default(&self) -> bool {
        self.inner.get("default").as_::<bool>()
    }
}
