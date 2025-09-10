use super::*;

/// The RTCSctpTransport class.
/// [`RTCSctpTransport`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSctpTransport)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCSctpTransport {
    inner: EventTarget,
}

impl FromVal for RTCSctpTransport {
    fn from_val(v: &Any) -> Self {
        RTCSctpTransport {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCSctpTransport {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCSctpTransport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCSctpTransport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCSctpTransport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCSctpTransport> for Any {
    fn from(s: RTCSctpTransport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCSctpTransport> for Any {
    fn from(s: &RTCSctpTransport) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCSctpTransport);

impl RTCSctpTransport {
    /// Getter of the `transport` attribute.
    /// [`RTCSctpTransport.transport`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSctpTransport/transport)
    pub fn transport(&self) -> RTCDtlsTransport {
        self.inner.get("transport").as_::<RTCDtlsTransport>()
    }
}
impl RTCSctpTransport {
    /// Getter of the `state` attribute.
    /// [`RTCSctpTransport.state`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSctpTransport/state)
    pub fn state(&self) -> RTCSctpTransportState {
        self.inner.get("state").as_::<RTCSctpTransportState>()
    }
}
impl RTCSctpTransport {
    /// Getter of the `maxMessageSize` attribute.
    /// [`RTCSctpTransport.maxMessageSize`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSctpTransport/maxMessageSize)
    pub fn max_message_size(&self) -> f64 {
        self.inner.get("maxMessageSize").as_::<f64>()
    }
}
impl RTCSctpTransport {
    /// Getter of the `maxChannels` attribute.
    /// [`RTCSctpTransport.maxChannels`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSctpTransport/maxChannels)
    pub fn max_channels(&self) -> u16 {
        self.inner.get("maxChannels").as_::<u16>()
    }
}
impl RTCSctpTransport {
    /// Getter of the `onstatechange` attribute.
    /// [`RTCSctpTransport.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSctpTransport/onstatechange)
    pub fn onstatechange(&self) -> Any {
        self.inner.get("onstatechange").as_::<Any>()
    }

    /// Setter of the `onstatechange` attribute.
    /// [`RTCSctpTransport.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSctpTransport/onstatechange)
    pub fn set_onstatechange(&mut self, value: &Any) {
        self.inner.set("onstatechange", value);
    }
}
