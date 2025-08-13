use super::*;




/// The SpeechRecognitionEvent class.
/// [`SpeechRecognitionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionEvent {
    inner: Event,
}

impl FromVal for SpeechRecognitionEvent {
    fn from_val(v: &Any) -> Self {
        SpeechRecognitionEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for SpeechRecognitionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechRecognitionEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SpeechRecognitionEvent> for Any {
    fn from(s: SpeechRecognitionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechRecognitionEvent> for Any {
    fn from(s: &SpeechRecognitionEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechRecognitionEvent);



impl SpeechRecognitionEvent {
    /// The `new SpeechRecognitionEvent(..)` constructor, creating a new SpeechRecognitionEvent instance
    pub fn new(type_: &JsString, event_init_dict: &SpeechRecognitionEventInit) -> SpeechRecognitionEvent {
        Self {
            inner: Any::global("SpeechRecognitionEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl SpeechRecognitionEvent {
    /// Getter of the `resultIndex` attribute.
    /// [`SpeechRecognitionEvent.resultIndex`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/resultIndex)
    pub fn result_index(&self) -> u32 {
        self.inner.get("resultIndex").as_::<u32>()
    }

}
impl SpeechRecognitionEvent {
    /// Getter of the `results` attribute.
    /// [`SpeechRecognitionEvent.results`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionEvent/results)
    pub fn results(&self) -> SpeechRecognitionResultList {
        self.inner.get("results").as_::<SpeechRecognitionResultList>()
    }

}
