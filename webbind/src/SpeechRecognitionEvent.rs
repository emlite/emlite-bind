use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeechRecognitionEvent {
    inner: Event,
}
impl FromVal for SpeechRecognitionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechRecognitionEvent {
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
impl core::ops::Deref for SpeechRecognitionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechRecognitionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechRecognitionEvent> for emlite::Val {
    fn from(s: SpeechRecognitionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechRecognitionEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> SpeechRecognitionEvent {
        Self {
            inner: emlite::Val::global("SpeechRecognitionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl SpeechRecognitionEvent {
    pub fn result_index(&self) -> u32 {
        self.inner.get("resultIndex").as_::<u32>()
    }
}
impl SpeechRecognitionEvent {
    pub fn results(&self) -> SpeechRecognitionResultList {
        self.inner
            .get("results")
            .as_::<SpeechRecognitionResultList>()
    }
}
