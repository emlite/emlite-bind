use super::*;




/// The RTCDataChannelEvent class.
/// [`RTCDataChannelEvent`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDataChannelEvent {
    inner: Event,
}

impl FromVal for RTCDataChannelEvent {
    fn from_val(v: &Any) -> Self {
        RTCDataChannelEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCDataChannelEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCDataChannelEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCDataChannelEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCDataChannelEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCDataChannelEvent> for Any {
    fn from(s: RTCDataChannelEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCDataChannelEvent> for Any {
    fn from(s: &RTCDataChannelEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCDataChannelEvent);



impl RTCDataChannelEvent {
    /// The `new RTCDataChannelEvent(..)` constructor, creating a new RTCDataChannelEvent instance
    pub fn new(type_: &JsString, event_init_dict: &RTCDataChannelEventInit) -> RTCDataChannelEvent {
        Self {
            inner: Any::global("RTCDataChannelEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl RTCDataChannelEvent {
    /// Getter of the `channel` attribute.
    /// [`RTCDataChannelEvent.channel`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDataChannelEvent/channel)
    pub fn channel(&self) -> RTCDataChannel {
        self.inner.get("channel").as_::<RTCDataChannel>()
    }

}
