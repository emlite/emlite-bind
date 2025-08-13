use super::*;




/// The MediaStreamConstraints dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamConstraints {
    inner: Any,
}

impl FromVal for MediaStreamConstraints {
    fn from_val(v: &Any) -> Self {
        MediaStreamConstraints { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaStreamConstraints {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaStreamConstraints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaStreamConstraints {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaStreamConstraints {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaStreamConstraints> for Any {
    fn from(s: MediaStreamConstraints) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaStreamConstraints> for Any {
    fn from(s: &MediaStreamConstraints) -> Any {
        s.inner.clone()
    }
}

impl MediaStreamConstraints {
    /// Getter of the `peerIdentity` attribute.
    pub fn peer_identity(&self) -> JsString {
        self.inner.get("peerIdentity").as_::<JsString>()
    }

    /// Setter of the `peerIdentity` attribute.
    pub fn set_peer_identity(&mut self, value: &JsString) {
        self.inner.set("peerIdentity", value);
    }
}
