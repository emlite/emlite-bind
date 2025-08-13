use super::*;




/// The MediaCapabilitiesEncodingInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaCapabilitiesEncodingInfo {
    inner: Any,
}

impl FromVal for MediaCapabilitiesEncodingInfo {
    fn from_val(v: &Any) -> Self {
        MediaCapabilitiesEncodingInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaCapabilitiesEncodingInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaCapabilitiesEncodingInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaCapabilitiesEncodingInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaCapabilitiesEncodingInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaCapabilitiesEncodingInfo> for Any {
    fn from(s: MediaCapabilitiesEncodingInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaCapabilitiesEncodingInfo> for Any {
    fn from(s: &MediaCapabilitiesEncodingInfo) -> Any {
        s.inner.clone()
    }
}

impl MediaCapabilitiesEncodingInfo {
    /// Getter of the `configuration` attribute.
    pub fn configuration(&self) -> MediaEncodingConfiguration {
        self.inner.get("configuration").as_::<MediaEncodingConfiguration>()
    }

    /// Setter of the `configuration` attribute.
    pub fn set_configuration(&mut self, value: &MediaEncodingConfiguration) {
        self.inner.set("configuration", value);
    }
}
