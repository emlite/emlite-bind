use super::*;




/// The RTCCertificate class.
/// [`RTCCertificate`](https://developer.mozilla.org/en-US/docs/Web/API/RTCCertificate)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCCertificate {
    inner: Any,
}

impl FromVal for RTCCertificate {
    fn from_val(v: &Any) -> Self {
        RTCCertificate { inner: Any::from_val(v) }
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
    pub fn get_fingerprints(&self, ) -> TypedArray<RTCDtlsFingerprint> {
        self.inner.call("getFingerprints", &[]).as_::<TypedArray<RTCDtlsFingerprint>>()
    }
}
