use super::*;

#[derive(Clone, Debug)]
pub struct RTCSessionDescription {
    inner: emlite::Val,
}
impl FromVal for RTCSessionDescription {
    fn from_val(v: &emlite::Val) -> Self {
        RTCSessionDescription {
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
impl std::ops::Deref for RTCSessionDescription {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for RTCSessionDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<RTCSessionDescription> for emlite::Val {
    fn from(s: RTCSessionDescription) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl RTCSessionDescription {
    pub fn new(description_init_dict: RTCSessionDescriptionInit) -> RTCSessionDescription {
        Self {
            inner: emlite::Val::global("RTCSessionDescription")
                .new(&[description_init_dict.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl RTCSessionDescription {
    pub fn type_(&self) -> RTCSdpType {
        self.inner.get("type").as_::<RTCSdpType>()
    }
}
impl RTCSessionDescription {
    pub fn sdp(&self) -> jsbind::DOMString {
        self.inner.get("sdp").as_::<jsbind::DOMString>()
    }
}
impl RTCSessionDescription {
    pub fn to_json(&self) -> RTCSessionDescriptionInit {
        self.inner
            .call("toJSON", &[])
            .as_::<RTCSessionDescriptionInit>()
    }
}
