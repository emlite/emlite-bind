use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for SpeechSynthesisVoice {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechSynthesisVoice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SpeechSynthesisVoice {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SpeechSynthesisVoice {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SpeechSynthesisVoice> for emlite::Val {
    fn from(s: SpeechSynthesisVoice) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SpeechSynthesisVoice> for emlite::Val {
    fn from(s: &SpeechSynthesisVoice) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SpeechSynthesisVoice);

impl SpeechSynthesisVoice {
    pub fn voice_uri(&self) -> String {
        self.inner.get("voiceURI").as_::<String>()
    }
}
impl SpeechSynthesisVoice {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl SpeechSynthesisVoice {
    pub fn lang(&self) -> String {
        self.inner.get("lang").as_::<String>()
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
