use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeechRecognitionErrorEvent {
    inner: Event,
}
impl FromVal for SpeechRecognitionErrorEvent {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechRecognitionErrorEvent {
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
impl core::ops::Deref for SpeechRecognitionErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechRecognitionErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechRecognitionErrorEvent> for emlite::Val {
    fn from(s: SpeechRecognitionErrorEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechRecognitionErrorEvent {
    pub fn new(
        type_: jsbind::DOMString,
        event_init_dict: jsbind::Any,
    ) -> SpeechRecognitionErrorEvent {
        Self {
            inner: emlite::Val::global("SpeechRecognitionErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl SpeechRecognitionErrorEvent {
    pub fn error(&self) -> SpeechRecognitionErrorCode {
        self.inner.get("error").as_::<SpeechRecognitionErrorCode>()
    }
}
impl SpeechRecognitionErrorEvent {
    pub fn message(&self) -> jsbind::DOMString {
        self.inner.get("message").as_::<jsbind::DOMString>()
    }
}
