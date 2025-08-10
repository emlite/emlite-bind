use super::*;

/// The RTCDtlsFingerprint dictionary.
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
    /// Getter of the `algorithm` attribute.
    pub fn algorithm(&self) -> JsString {
        self.inner.get("algorithm").as_::<JsString>()
    }

    /// Setter of the `algorithm` attribute.
    pub fn set_algorithm(&mut self, value: &JsString) {
        self.inner.set("algorithm", value);
    }
}
impl RTCDtlsFingerprint {
    /// Getter of the `value` attribute.
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
