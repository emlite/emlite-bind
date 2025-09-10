use super::*;

/// The MediaDecodingConfiguration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaDecodingConfiguration {
    inner: Any,
}

impl FromVal for MediaDecodingConfiguration {
    fn from_val(v: &Any) -> Self {
        MediaDecodingConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaDecodingConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaDecodingConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaDecodingConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaDecodingConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaDecodingConfiguration> for Any {
    fn from(s: MediaDecodingConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaDecodingConfiguration> for Any {
    fn from(s: &MediaDecodingConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MediaDecodingConfiguration {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> MediaDecodingType {
        self.inner.get("type").as_::<MediaDecodingType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &MediaDecodingType) {
        self.inner.set("type", value);
    }
}
impl MediaDecodingConfiguration {
    /// Getter of the `keySystemConfiguration` attribute.
    pub fn key_system_configuration(&self) -> MediaCapabilitiesKeySystemConfiguration {
        self.inner
            .get("keySystemConfiguration")
            .as_::<MediaCapabilitiesKeySystemConfiguration>()
    }

    /// Setter of the `keySystemConfiguration` attribute.
    pub fn set_key_system_configuration(
        &mut self,
        value: &MediaCapabilitiesKeySystemConfiguration,
    ) {
        self.inner.set("keySystemConfiguration", value);
    }
}
