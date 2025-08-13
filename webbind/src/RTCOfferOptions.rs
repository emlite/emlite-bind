use super::*;




/// The RTCOfferOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCOfferOptions {
    inner: Any,
}

impl FromVal for RTCOfferOptions {
    fn from_val(v: &Any) -> Self {
        RTCOfferOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCOfferOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCOfferOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCOfferOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCOfferOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCOfferOptions> for Any {
    fn from(s: RTCOfferOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCOfferOptions> for Any {
    fn from(s: &RTCOfferOptions) -> Any {
        s.inner.clone()
    }
}

impl RTCOfferOptions {
    /// Getter of the `offerToReceiveAudio` attribute.
    pub fn offer_to_receive_audio(&self) -> bool {
        self.inner.get("offerToReceiveAudio").as_::<bool>()
    }

    /// Setter of the `offerToReceiveAudio` attribute.
    pub fn set_offer_to_receive_audio(&mut self, value: bool) {
        self.inner.set("offerToReceiveAudio", value);
    }
}
impl RTCOfferOptions {
    /// Getter of the `offerToReceiveVideo` attribute.
    pub fn offer_to_receive_video(&self) -> bool {
        self.inner.get("offerToReceiveVideo").as_::<bool>()
    }

    /// Setter of the `offerToReceiveVideo` attribute.
    pub fn set_offer_to_receive_video(&mut self, value: bool) {
        self.inner.set("offerToReceiveVideo", value);
    }
}
