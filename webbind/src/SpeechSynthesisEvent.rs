use super::*;

/// The SpeechSynthesisEvent class.
/// [`SpeechSynthesisEvent`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechSynthesisEvent {
    inner: Event,
}

impl FromVal for SpeechSynthesisEvent {
    fn from_val(v: &Any) -> Self {
        SpeechSynthesisEvent {
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

impl AsRef<Any> for SpeechSynthesisEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechSynthesisEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SpeechSynthesisEvent> for Any {
    fn from(s: SpeechSynthesisEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechSynthesisEvent> for Any {
    fn from(s: &SpeechSynthesisEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechSynthesisEvent);

impl SpeechSynthesisEvent {
    /// Getter of the `utterance` attribute.
    /// [`SpeechSynthesisEvent.utterance`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/utterance)
    pub fn utterance(&self) -> SpeechSynthesisUtterance {
        self.inner
            .get("utterance")
            .as_::<SpeechSynthesisUtterance>()
    }
}
impl SpeechSynthesisEvent {
    /// Getter of the `charIndex` attribute.
    /// [`SpeechSynthesisEvent.charIndex`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charIndex)
    pub fn char_index(&self) -> u32 {
        self.inner.get("charIndex").as_::<u32>()
    }
}
impl SpeechSynthesisEvent {
    /// Getter of the `charLength` attribute.
    /// [`SpeechSynthesisEvent.charLength`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/charLength)
    pub fn char_length(&self) -> u32 {
        self.inner.get("charLength").as_::<u32>()
    }
}
impl SpeechSynthesisEvent {
    /// Getter of the `elapsedTime` attribute.
    /// [`SpeechSynthesisEvent.elapsedTime`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/elapsedTime)
    pub fn elapsed_time(&self) -> f32 {
        self.inner.get("elapsedTime").as_::<f32>()
    }
}
impl SpeechSynthesisEvent {
    /// Getter of the `name` attribute.
    /// [`SpeechSynthesisEvent.name`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisEvent/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}

impl SpeechSynthesisEvent {
    /// The `new SpeechSynthesisEvent(..)` constructor, creating a new SpeechSynthesisEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &SpeechSynthesisEventInit,
    ) -> SpeechSynthesisEvent {
        Self {
            inner: Any::global("SpeechSynthesisEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
