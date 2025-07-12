use super::*;

#[derive(Clone, Debug)]
pub struct RTCIceCandidatePair {
    inner: emlite::Val,
}
impl FromVal for RTCIceCandidatePair {
    fn from_val(v: &emlite::Val) -> Self {
        RTCIceCandidatePair {
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
impl std::ops::Deref for RTCIceCandidatePair {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCIceCandidatePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCIceCandidatePair> for emlite::Val {
    fn from(s: RTCIceCandidatePair) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
