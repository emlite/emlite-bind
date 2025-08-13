use super::*;




/// The RTCIceCandidatePair class.
/// [`RTCIceCandidatePair`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidatePair)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceCandidatePair {
    inner: Any,
}

impl FromVal for RTCIceCandidatePair {
    fn from_val(v: &Any) -> Self {
        RTCIceCandidatePair { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCIceCandidatePair {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCIceCandidatePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCIceCandidatePair {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCIceCandidatePair {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCIceCandidatePair> for Any {
    fn from(s: RTCIceCandidatePair) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCIceCandidatePair> for Any {
    fn from(s: &RTCIceCandidatePair) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCIceCandidatePair);


impl RTCIceCandidatePair {
    /// Getter of the `local` attribute.
    /// [`RTCIceCandidatePair.local`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidatePair/local)
    pub fn local(&self) -> RTCIceCandidate {
        self.inner.get("local").as_::<RTCIceCandidate>()
    }

}
impl RTCIceCandidatePair {
    /// Getter of the `remote` attribute.
    /// [`RTCIceCandidatePair.remote`](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidatePair/remote)
    pub fn remote(&self) -> RTCIceCandidate {
        self.inner.get("remote").as_::<RTCIceCandidate>()
    }

}
