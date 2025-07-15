use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCSessionDescription {
    inner: emlite::Val,
}
impl FromVal for RTCSessionDescription {
    fn from_val(v: &emlite::Val) -> Self {
        RTCSessionDescription { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCSessionDescription {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCSessionDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCSessionDescription {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCSessionDescription {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<RTCSessionDescription> for emlite::Val {
    fn from(s: RTCSessionDescription) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(RTCSessionDescription);



impl RTCSessionDescription {
    pub fn new(description_init_dict: RTCSessionDescriptionInit) -> RTCSessionDescription {
        Self {
            inner: emlite::Val::global("RTCSessionDescription").new(&[description_init_dict.into()]).as_::<emlite::Val>(),
        }
    }

}
impl RTCSessionDescription {
    pub fn type_(&self) -> RTCSdpType {
        self.inner.get("type").as_::<RTCSdpType>()
    }

}
impl RTCSessionDescription {
    pub fn sdp(&self) -> DOMString {
        self.inner.get("sdp").as_::<DOMString>()
    }

}
impl RTCSessionDescription {
    pub fn to_json(&self, ) -> RTCSessionDescriptionInit {
        self.inner.call("toJSON", &[]).as_::<RTCSessionDescriptionInit>()
    }

}
