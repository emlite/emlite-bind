use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeechSynthesisErrorEvent {
    inner: SpeechSynthesisEvent,
}
impl FromVal for SpeechSynthesisErrorEvent {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechSynthesisErrorEvent {
            inner: SpeechSynthesisEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SpeechSynthesisErrorEvent {
    type Target = SpeechSynthesisEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechSynthesisErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechSynthesisErrorEvent> for emlite::Val {
    fn from(s: SpeechSynthesisErrorEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechSynthesisErrorEvent {
    pub fn new(
        type_: jsbind::DOMString,
        event_init_dict: jsbind::Any,
    ) -> SpeechSynthesisErrorEvent {
        Self {
            inner: emlite::Val::global("SpeechSynthesisErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<SpeechSynthesisEvent>(),
        }
    }
}
impl SpeechSynthesisErrorEvent {
    pub fn error(&self) -> SpeechSynthesisErrorCode {
        self.inner.get("error").as_::<SpeechSynthesisErrorCode>()
    }
}
