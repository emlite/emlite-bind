use super::*;

/// The SpeechRecognitionErrorEvent class.
/// [`SpeechRecognitionErrorEvent`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionErrorEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionErrorEvent {
    inner: Event,
}

impl FromVal for SpeechRecognitionErrorEvent {
    fn from_val(v: &Any) -> Self {
        SpeechRecognitionErrorEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for SpeechRecognitionErrorEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechRecognitionErrorEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SpeechRecognitionErrorEvent> for Any {
    fn from(s: SpeechRecognitionErrorEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechRecognitionErrorEvent> for Any {
    fn from(s: &SpeechRecognitionErrorEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechRecognitionErrorEvent);

impl SpeechRecognitionErrorEvent {
    /// The `new SpeechRecognitionErrorEvent(..)` constructor, creating a new SpeechRecognitionErrorEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &SpeechRecognitionErrorEventInit,
    ) -> SpeechRecognitionErrorEvent {
        Self {
            inner: Any::global("SpeechRecognitionErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl SpeechRecognitionErrorEvent {
    /// Getter of the `error` attribute.
    /// [`SpeechRecognitionErrorEvent.error`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionErrorEvent/error)
    pub fn error(&self) -> SpeechRecognitionErrorCode {
        self.inner.get("error").as_::<SpeechRecognitionErrorCode>()
    }
}
impl SpeechRecognitionErrorEvent {
    /// Getter of the `message` attribute.
    /// [`SpeechRecognitionErrorEvent.message`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionErrorEvent/message)
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }
}
