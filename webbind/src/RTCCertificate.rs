use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCDtlsFingerprint {
    inner: Any,
}
impl FromVal for RTCDtlsFingerprint {
    fn from_val(v: &Any) -> Self {
        RTCDtlsFingerprint { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCDtlsFingerprint {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCDtlsFingerprint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCDtlsFingerprint {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCDtlsFingerprint {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCDtlsFingerprint> for Any {
    fn from(s: RTCDtlsFingerprint) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCDtlsFingerprint> for Any {
    fn from(s: &RTCDtlsFingerprint) -> Any {
        s.inner.clone()
    }
}

impl RTCDtlsFingerprint {
    pub fn algorithm(&self) -> DOMString {
        self.inner.get("algorithm").as_::<DOMString>()
    }

    pub fn set_algorithm(&mut self, value: &DOMString) {
        self.inner.set("algorithm", value);
    }
}
impl RTCDtlsFingerprint {
    pub fn value(&self) -> DOMString {
        self.inner.get("value").as_::<DOMString>()
    }

    pub fn set_value(&mut self, value: &DOMString) {
        self.inner.set("value", value);
    }
}
/// The RTCCertificate class.
/// [`RTCCertificate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCCertificate {
    inner: Any,
}
impl FromVal for RTCCertificate {
    fn from_val(v: &Any) -> Self {
        RTCCertificate {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCCertificate {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCCertificate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCCertificate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCCertificate {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCCertificate> for Any {
    fn from(s: RTCCertificate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCCertificate> for Any {
    fn from(s: &RTCCertificate) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCCertificate);

impl RTCCertificate {
    /// Getter of the `expires` attribute.
    /// [`RTCCertificate.expires`](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate/expires)
    pub fn expires(&self) -> Any {
        self.inner.get("expires").as_::<Any>()
    }
}
impl RTCCertificate {
    /// The getFingerprints method.
    /// [`RTCCertificate.getFingerprints`](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate/getFingerprints)
    pub fn get_fingerprints(&self) -> Sequence<RTCDtlsFingerprint> {
        self.inner
            .call("getFingerprints", &[])
            .as_::<Sequence<RTCDtlsFingerprint>>()
    }
}
