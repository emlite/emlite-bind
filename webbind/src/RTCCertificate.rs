use super::*;

#[derive(Clone, Debug)]
pub struct RTCDtlsFingerprint {
    inner: emlite::Val,
}
impl FromVal for RTCDtlsFingerprint {
    fn from_val(v: &emlite::Val) -> Self {
        RTCDtlsFingerprint { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCDtlsFingerprint {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCDtlsFingerprint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCDtlsFingerprint> for emlite::Val {
    fn from(s: RTCDtlsFingerprint) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCDtlsFingerprint {
    pub fn algorithm(&self) -> jsbind::DOMString {
        self.inner.get("algorithm").as_::<jsbind::DOMString>()
    }

    pub fn set_algorithm(&mut self, value: jsbind::DOMString) {
        self.inner.set("algorithm", value);
    }
}
impl RTCDtlsFingerprint {
    pub fn value(&self) -> jsbind::DOMString {
        self.inner.get("value").as_::<jsbind::DOMString>()
    }

    pub fn set_value(&mut self, value: jsbind::DOMString) {
        self.inner.set("value", value);
    }
}
#[derive(Clone, Debug)]
pub struct RTCCertificate {
    inner: emlite::Val,
}
impl FromVal for RTCCertificate {
    fn from_val(v: &emlite::Val) -> Self {
        RTCCertificate {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for RTCCertificate {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCCertificate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCCertificate> for emlite::Val {
    fn from(s: RTCCertificate) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCCertificate {
    pub fn expires(&self) -> jsbind::Any {
        self.inner.get("expires").as_::<jsbind::Any>()
    }
}
impl RTCCertificate {
    pub fn get_fingerprints(&self) -> jsbind::Sequence<RTCDtlsFingerprint> {
        self.inner
            .call("getFingerprints", &[])
            .as_::<jsbind::Sequence<RTCDtlsFingerprint>>()
    }
}
