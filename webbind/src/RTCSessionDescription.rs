use super::*;




/// The RTCSessionDescription class.
/// [`RTCSessionDescription`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCSessionDescription {
    inner: Any,
}

impl FromVal for RTCSessionDescription {
    fn from_val(v: &Any) -> Self {
        RTCSessionDescription { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCSessionDescription {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCSessionDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCSessionDescription {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCSessionDescription {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCSessionDescription> for Any {
    fn from(s: RTCSessionDescription) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCSessionDescription> for Any {
    fn from(s: &RTCSessionDescription) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCSessionDescription);



impl RTCSessionDescription {
    /// The `new RTCSessionDescription(..)` constructor, creating a new RTCSessionDescription instance
    pub fn new(description_init_dict: &RTCSessionDescriptionInit) -> RTCSessionDescription {
        Self {
            inner: Any::global("RTCSessionDescription").new(&[description_init_dict.into()]).as_::<Any>(),
        }
    }

}
impl RTCSessionDescription {
    /// Getter of the `type` attribute.
    /// [`RTCSessionDescription.type`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/type)
    pub fn type_(&self) -> RTCSdpType {
        self.inner.get("type").as_::<RTCSdpType>()
    }

}
impl RTCSessionDescription {
    /// Getter of the `sdp` attribute.
    /// [`RTCSessionDescription.sdp`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/sdp)
    pub fn sdp(&self) -> JsString {
        self.inner.get("sdp").as_::<JsString>()
    }

}
impl RTCSessionDescription {
    /// The toJSON method.
    /// [`RTCSessionDescription.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/RTCSessionDescription/toJSON)
    pub fn to_json(&self, ) -> RTCSessionDescriptionInit {
        self.inner.call("toJSON", &[]).as_::<RTCSessionDescriptionInit>()
    }
}
