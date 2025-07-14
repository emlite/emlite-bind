use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeechSynthesisEvent {
    inner: Event,
}
impl FromVal for SpeechSynthesisEvent {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechSynthesisEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SpeechSynthesisEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechSynthesisEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechSynthesisEvent> for emlite::Val {
    fn from(s: SpeechSynthesisEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechSynthesisEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> SpeechSynthesisEvent {
        Self {
            inner: emlite::Val::global("SpeechSynthesisEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl SpeechSynthesisEvent {
    pub fn utterance(&self) -> SpeechSynthesisUtterance {
        self.inner
            .get("utterance")
            .as_::<SpeechSynthesisUtterance>()
    }
}
impl SpeechSynthesisEvent {
    pub fn char_index(&self) -> u32 {
        self.inner.get("charIndex").as_::<u32>()
    }
}
impl SpeechSynthesisEvent {
    pub fn char_length(&self) -> u32 {
        self.inner.get("charLength").as_::<u32>()
    }
}
impl SpeechSynthesisEvent {
    pub fn elapsed_time(&self) -> f32 {
        self.inner.get("elapsedTime").as_::<f32>()
    }
}
impl SpeechSynthesisEvent {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
