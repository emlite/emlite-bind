use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for SpeechRecognitionErrorEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SpeechRecognitionErrorEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&SpeechRecognitionErrorEvent> for emlite::Val {
    fn from(s: &SpeechRecognitionErrorEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SpeechRecognitionErrorEvent);

impl SpeechRecognitionErrorEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> SpeechRecognitionErrorEvent {
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
    pub fn message(&self) -> DOMString {
        self.inner.get("message").as_::<DOMString>()
    }
}
