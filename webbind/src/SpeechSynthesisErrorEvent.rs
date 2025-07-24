use super::*;

/// The SpeechSynthesisErrorEvent class.
/// [`SpeechSynthesisErrorEvent`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechSynthesisErrorEvent {
    inner: SpeechSynthesisEvent,
}
impl FromVal for SpeechSynthesisErrorEvent {
    fn from_val(v: &Any) -> Self {
        SpeechSynthesisErrorEvent {
            inner: SpeechSynthesisEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for SpeechSynthesisErrorEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SpeechSynthesisErrorEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SpeechSynthesisErrorEvent> for Any {
    fn from(s: SpeechSynthesisErrorEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SpeechSynthesisErrorEvent> for Any {
    fn from(s: &SpeechSynthesisErrorEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SpeechSynthesisErrorEvent);

impl SpeechSynthesisErrorEvent {
    /// The `new SpeechSynthesisErrorEvent(..)` constructor, creating a new SpeechSynthesisErrorEvent instance
    pub fn new(type_: &DOMString, event_init_dict: &Any) -> SpeechSynthesisErrorEvent {
        Self {
            inner: Any::global("SpeechSynthesisErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<SpeechSynthesisEvent>(),
        }
    }
}
impl SpeechSynthesisErrorEvent {
    /// Getter of the `error` attribute.
    /// [`SpeechSynthesisErrorEvent.error`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisErrorEvent/error)
    pub fn error(&self) -> SpeechSynthesisErrorCode {
        self.inner.get("error").as_::<SpeechSynthesisErrorCode>()
    }
}
