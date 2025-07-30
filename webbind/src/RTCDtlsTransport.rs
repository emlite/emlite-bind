use super::*;

/// The RTCDtlsTransport class.
/// [`RTCDtlsTransport`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDtlsTransport {
    inner: EventTarget,
}
impl FromVal for RTCDtlsTransport {
    fn from_val(v: &Any) -> Self {
        RTCDtlsTransport {
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
impl core::ops::Deref for RTCDtlsTransport {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDtlsTransport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCDtlsTransport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCDtlsTransport {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCDtlsTransport> for Any {
    fn from(s: RTCDtlsTransport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCDtlsTransport> for Any {
    fn from(s: &RTCDtlsTransport) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCDtlsTransport);

impl RTCDtlsTransport {
    /// Getter of the `iceTransport` attribute.
    /// [`RTCDtlsTransport.iceTransport`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport/iceTransport)
    pub fn ice_transport(&self) -> RTCIceTransport {
        self.inner.get("iceTransport").as_::<RTCIceTransport>()
    }
}
impl RTCDtlsTransport {
    /// Getter of the `state` attribute.
    /// [`RTCDtlsTransport.state`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport/state)
    pub fn state(&self) -> RTCDtlsTransportState {
        self.inner.get("state").as_::<RTCDtlsTransportState>()
    }
}
impl RTCDtlsTransport {
    /// The getRemoteCertificates method.
    /// [`RTCDtlsTransport.getRemoteCertificates`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport/getRemoteCertificates)
    pub fn get_remote_certificates(&self) -> TypedArray<ArrayBuffer> {
        self.inner
            .call("getRemoteCertificates", &[])
            .as_::<TypedArray<ArrayBuffer>>()
    }
}
impl RTCDtlsTransport {
    /// Getter of the `onstatechange` attribute.
    /// [`RTCDtlsTransport.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport/onstatechange)
    pub fn onstatechange(&self) -> Any {
        self.inner.get("onstatechange").as_::<Any>()
    }

    /// Setter of the `onstatechange` attribute.
    /// [`RTCDtlsTransport.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport/onstatechange)
    pub fn set_onstatechange(&mut self, value: &Any) {
        self.inner.set("onstatechange", value);
    }
}
impl RTCDtlsTransport {
    /// Getter of the `onerror` attribute.
    /// [`RTCDtlsTransport.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`RTCDtlsTransport.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
