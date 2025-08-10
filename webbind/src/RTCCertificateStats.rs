use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCCertificateStats {
    inner: Any,
}
impl FromVal for RTCCertificateStats {
    fn from_val(v: &Any) -> Self {
        RTCCertificateStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCCertificateStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCCertificateStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCCertificateStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCCertificateStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCCertificateStats> for Any {
    fn from(s: RTCCertificateStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCCertificateStats> for Any {
    fn from(s: &RTCCertificateStats) -> Any {
        s.inner.clone()
    }
}

impl RTCCertificateStats {
    pub fn fingerprint(&self) -> JsString {
        self.inner.get("fingerprint").as_::<JsString>()
    }

    pub fn set_fingerprint(&mut self, value: &JsString) {
        self.inner.set("fingerprint", value);
    }
}
impl RTCCertificateStats {
    pub fn fingerprint_algorithm(&self) -> JsString {
        self.inner.get("fingerprintAlgorithm").as_::<JsString>()
    }

    pub fn set_fingerprint_algorithm(&mut self, value: &JsString) {
        self.inner.set("fingerprintAlgorithm", value);
    }
}
impl RTCCertificateStats {
    pub fn base64_certificate(&self) -> JsString {
        self.inner.get("base64Certificate").as_::<JsString>()
    }

    pub fn set_base64_certificate(&mut self, value: &JsString) {
        self.inner.set("base64Certificate", value);
    }
}
impl RTCCertificateStats {
    pub fn issuer_certificate_id(&self) -> JsString {
        self.inner.get("issuerCertificateId").as_::<JsString>()
    }

    pub fn set_issuer_certificate_id(&mut self, value: &JsString) {
        self.inner.set("issuerCertificateId", value);
    }
}
