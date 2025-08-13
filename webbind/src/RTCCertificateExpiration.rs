use super::*;




/// The RTCCertificateExpiration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCCertificateExpiration {
    inner: Any,
}

impl FromVal for RTCCertificateExpiration {
    fn from_val(v: &Any) -> Self {
        RTCCertificateExpiration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCCertificateExpiration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCCertificateExpiration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCCertificateExpiration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCCertificateExpiration {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCCertificateExpiration> for Any {
    fn from(s: RTCCertificateExpiration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCCertificateExpiration> for Any {
    fn from(s: &RTCCertificateExpiration) -> Any {
        s.inner.clone()
    }
}

impl RTCCertificateExpiration {
    /// Getter of the `expires` attribute.
    pub fn expires(&self) -> u64 {
        self.inner.get("expires").as_::<u64>()
    }

    /// Setter of the `expires` attribute.
    pub fn set_expires(&mut self, value: u64) {
        self.inner.set("expires", value);
    }
}
