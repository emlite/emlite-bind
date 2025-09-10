use super::*;

/// The RTCPeerConnectionIceErrorEvent class.
/// [`RTCPeerConnectionIceErrorEvent`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCPeerConnectionIceErrorEvent {
    inner: Event,
}

impl FromVal for RTCPeerConnectionIceErrorEvent {
    fn from_val(v: &Any) -> Self {
        RTCPeerConnectionIceErrorEvent {
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

impl core::ops::Deref for RTCPeerConnectionIceErrorEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCPeerConnectionIceErrorEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCPeerConnectionIceErrorEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCPeerConnectionIceErrorEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCPeerConnectionIceErrorEvent> for Any {
    fn from(s: RTCPeerConnectionIceErrorEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCPeerConnectionIceErrorEvent> for Any {
    fn from(s: &RTCPeerConnectionIceErrorEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCPeerConnectionIceErrorEvent);

impl RTCPeerConnectionIceErrorEvent {
    /// The `new RTCPeerConnectionIceErrorEvent(..)` constructor, creating a new RTCPeerConnectionIceErrorEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &RTCPeerConnectionIceErrorEventInit,
    ) -> RTCPeerConnectionIceErrorEvent {
        Self {
            inner: Any::global("RTCPeerConnectionIceErrorEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl RTCPeerConnectionIceErrorEvent {
    /// Getter of the `address` attribute.
    /// [`RTCPeerConnectionIceErrorEvent.address`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/address)
    pub fn address(&self) -> JsString {
        self.inner.get("address").as_::<JsString>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    /// Getter of the `port` attribute.
    /// [`RTCPeerConnectionIceErrorEvent.port`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/port)
    pub fn port(&self) -> u16 {
        self.inner.get("port").as_::<u16>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    /// Getter of the `url` attribute.
    /// [`RTCPeerConnectionIceErrorEvent.url`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    /// Getter of the `errorCode` attribute.
    /// [`RTCPeerConnectionIceErrorEvent.errorCode`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/errorCode)
    pub fn error_code(&self) -> u16 {
        self.inner.get("errorCode").as_::<u16>()
    }
}
impl RTCPeerConnectionIceErrorEvent {
    /// Getter of the `errorText` attribute.
    /// [`RTCPeerConnectionIceErrorEvent.errorText`](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/errorText)
    pub fn error_text(&self) -> JsString {
        self.inner.get("errorText").as_::<JsString>()
    }
}
