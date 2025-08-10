use super::*;

/// The OfflineAudioCompletionEvent class.
/// [`OfflineAudioCompletionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OfflineAudioCompletionEvent {
    inner: Event,
}
impl FromVal for OfflineAudioCompletionEvent {
    fn from_val(v: &Any) -> Self {
        OfflineAudioCompletionEvent {
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
impl core::ops::Deref for OfflineAudioCompletionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OfflineAudioCompletionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OfflineAudioCompletionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OfflineAudioCompletionEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OfflineAudioCompletionEvent> for Any {
    fn from(s: OfflineAudioCompletionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OfflineAudioCompletionEvent> for Any {
    fn from(s: &OfflineAudioCompletionEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OfflineAudioCompletionEvent);

impl OfflineAudioCompletionEvent {
    /// The `new OfflineAudioCompletionEvent(..)` constructor, creating a new OfflineAudioCompletionEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &OfflineAudioCompletionEventInit,
    ) -> OfflineAudioCompletionEvent {
        Self {
            inner: Any::global("OfflineAudioCompletionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl OfflineAudioCompletionEvent {
    /// Getter of the `renderedBuffer` attribute.
    /// [`OfflineAudioCompletionEvent.renderedBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/OfflineAudioCompletionEvent/renderedBuffer)
    pub fn rendered_buffer(&self) -> AudioBuffer {
        self.inner.get("renderedBuffer").as_::<AudioBuffer>()
    }
}
