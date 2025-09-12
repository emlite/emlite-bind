use super::*;

/// The RTCPeerConnectionIceEvent class.
/// [`RTCPeerConnectionIceEvent`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCPeerConnectionIceEvent {
    inner: Event,
}

impl FromVal for RTCPeerConnectionIceEvent {
    fn from_val(v: &Any) -> Self {
        RTCPeerConnectionIceEvent {
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

impl core::ops::Deref for RTCPeerConnectionIceEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCPeerConnectionIceEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCPeerConnectionIceEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCPeerConnectionIceEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCPeerConnectionIceEvent> for Any {
    fn from(s: RTCPeerConnectionIceEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCPeerConnectionIceEvent> for Any {
    fn from(s: &RTCPeerConnectionIceEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCPeerConnectionIceEvent);

impl RTCPeerConnectionIceEvent {
    /// Getter of the `candidate` attribute.
    /// [`RTCPeerConnectionIceEvent.candidate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/candidate)
    pub fn candidate(&self) -> RTCIceCandidate {
        self.inner.get("candidate").as_::<RTCIceCandidate>()
    }
}
impl RTCPeerConnectionIceEvent {
    /// Getter of the `url` attribute.
    /// [`RTCPeerConnectionIceEvent.url`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceEvent/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}

impl RTCPeerConnectionIceEvent {
    /// The `new RTCPeerConnectionIceEvent(..)` constructor, creating a new RTCPeerConnectionIceEvent instance
    pub fn new(type_: &JsString) -> RTCPeerConnectionIceEvent {
        Self {
            inner: Any::global("RTCPeerConnectionIceEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }
}

impl RTCPeerConnectionIceEvent {
    /// The `new RTCPeerConnectionIceEvent(..)` constructor, creating a new RTCPeerConnectionIceEvent instance
    pub fn new_with_event_init_dict(
        type_: &JsString,
        event_init_dict: &RTCPeerConnectionIceEventInit,
    ) -> RTCPeerConnectionIceEvent {
        Self {
            inner: Any::global("RTCPeerConnectionIceEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
