use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCIceCandidatePair {
    inner: emlite::Val,
}
impl FromVal for RTCIceCandidatePair {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceCandidatePair { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCIceCandidatePair {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCIceCandidatePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCIceCandidatePair {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCIceCandidatePair {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<RTCIceCandidatePair> for emlite::Val {
    fn from(s: RTCIceCandidatePair) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(RTCIceCandidatePair);


impl RTCIceCandidatePair {
    pub fn local(&self) -> RTCIceCandidate {
        self.inner.get("local").as_::<RTCIceCandidate>()
    }

}
impl RTCIceCandidatePair {
    pub fn remote(&self) -> RTCIceCandidate {
        self.inner.get("remote").as_::<RTCIceCandidate>()
    }

}
