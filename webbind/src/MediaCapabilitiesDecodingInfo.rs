use super::*;




/// The MediaCapabilitiesDecodingInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaCapabilitiesDecodingInfo {
    inner: Any,
}

impl FromVal for MediaCapabilitiesDecodingInfo {
    fn from_val(v: &Any) -> Self {
        MediaCapabilitiesDecodingInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaCapabilitiesDecodingInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaCapabilitiesDecodingInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaCapabilitiesDecodingInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaCapabilitiesDecodingInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaCapabilitiesDecodingInfo> for Any {
    fn from(s: MediaCapabilitiesDecodingInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaCapabilitiesDecodingInfo> for Any {
    fn from(s: &MediaCapabilitiesDecodingInfo) -> Any {
        s.inner.clone()
    }
}

impl MediaCapabilitiesDecodingInfo {
    /// Getter of the `keySystemAccess` attribute.
    pub fn key_system_access(&self) -> MediaKeySystemAccess {
        self.inner.get("keySystemAccess").as_::<MediaKeySystemAccess>()
    }

    /// Setter of the `keySystemAccess` attribute.
    pub fn set_key_system_access(&mut self, value: &MediaKeySystemAccess) {
        self.inner.set("keySystemAccess", value);
    }
}
impl MediaCapabilitiesDecodingInfo {
    /// Getter of the `configuration` attribute.
    pub fn configuration(&self) -> MediaDecodingConfiguration {
        self.inner.get("configuration").as_::<MediaDecodingConfiguration>()
    }

    /// Setter of the `configuration` attribute.
    pub fn set_configuration(&mut self, value: &MediaDecodingConfiguration) {
        self.inner.set("configuration", value);
    }
}
