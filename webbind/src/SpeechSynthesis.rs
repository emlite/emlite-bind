use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechSynthesis {
    inner: EventTarget,
}
impl FromVal for SpeechSynthesis {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechSynthesis {
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
impl core::ops::Deref for SpeechSynthesis {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechSynthesis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SpeechSynthesis {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SpeechSynthesis {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SpeechSynthesis> for emlite::Val {
    fn from(s: SpeechSynthesis) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SpeechSynthesis> for emlite::Val {
    fn from(s: &SpeechSynthesis) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SpeechSynthesis);

impl SpeechSynthesis {
    pub fn pending(&self) -> bool {
        self.inner.get("pending").as_::<bool>()
    }
}
impl SpeechSynthesis {
    pub fn speaking(&self) -> bool {
        self.inner.get("speaking").as_::<bool>()
    }
}
impl SpeechSynthesis {
    pub fn paused(&self) -> bool {
        self.inner.get("paused").as_::<bool>()
    }
}
impl SpeechSynthesis {
    pub fn onvoiceschanged(&self) -> Any {
        self.inner.get("onvoiceschanged").as_::<Any>()
    }

    pub fn set_onvoiceschanged(&mut self, value: Any) {
        self.inner.set("onvoiceschanged", value);
    }
}
impl SpeechSynthesis {
    pub fn speak(&self, utterance: SpeechSynthesisUtterance) -> Undefined {
        self.inner
            .call("speak", &[utterance.into()])
            .as_::<Undefined>()
    }
}
impl SpeechSynthesis {
    pub fn cancel(&self) -> Undefined {
        self.inner.call("cancel", &[]).as_::<Undefined>()
    }
}
impl SpeechSynthesis {
    pub fn pause(&self) -> Undefined {
        self.inner.call("pause", &[]).as_::<Undefined>()
    }
}
impl SpeechSynthesis {
    pub fn resume(&self) -> Undefined {
        self.inner.call("resume", &[]).as_::<Undefined>()
    }
}
impl SpeechSynthesis {
    pub fn get_voices(&self) -> Sequence<SpeechSynthesisVoice> {
        self.inner
            .call("getVoices", &[])
            .as_::<Sequence<SpeechSynthesisVoice>>()
    }
}
