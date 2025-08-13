use super::*;




/// The RTCTrackEvent class.
/// [`RTCTrackEvent`](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCTrackEvent {
    inner: Event,
}

impl FromVal for RTCTrackEvent {
    fn from_val(v: &Any) -> Self {
        RTCTrackEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCTrackEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCTrackEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCTrackEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCTrackEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCTrackEvent> for Any {
    fn from(s: RTCTrackEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCTrackEvent> for Any {
    fn from(s: &RTCTrackEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCTrackEvent);



impl RTCTrackEvent {
    /// The `new RTCTrackEvent(..)` constructor, creating a new RTCTrackEvent instance
    pub fn new(type_: &JsString, event_init_dict: &RTCTrackEventInit) -> RTCTrackEvent {
        Self {
            inner: Any::global("RTCTrackEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl RTCTrackEvent {
    /// Getter of the `receiver` attribute.
    /// [`RTCTrackEvent.receiver`](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/receiver)
    pub fn receiver(&self) -> RTCRtpReceiver {
        self.inner.get("receiver").as_::<RTCRtpReceiver>()
    }

}
impl RTCTrackEvent {
    /// Getter of the `track` attribute.
    /// [`RTCTrackEvent.track`](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/track)
    pub fn track(&self) -> MediaStreamTrack {
        self.inner.get("track").as_::<MediaStreamTrack>()
    }

}
impl RTCTrackEvent {
    /// Getter of the `streams` attribute.
    /// [`RTCTrackEvent.streams`](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/streams)
    pub fn streams(&self) -> TypedArray<MediaStream> {
        self.inner.get("streams").as_::<TypedArray<MediaStream>>()
    }

}
impl RTCTrackEvent {
    /// Getter of the `transceiver` attribute.
    /// [`RTCTrackEvent.transceiver`](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/transceiver)
    pub fn transceiver(&self) -> RTCRtpTransceiver {
        self.inner.get("transceiver").as_::<RTCRtpTransceiver>()
    }

}
