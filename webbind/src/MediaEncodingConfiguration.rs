use super::*;

/// The MediaEncodingConfiguration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaEncodingConfiguration {
    inner: Any,
}

impl FromVal for MediaEncodingConfiguration {
    fn from_val(v: &Any) -> Self {
        MediaEncodingConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaEncodingConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaEncodingConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaEncodingConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaEncodingConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaEncodingConfiguration> for Any {
    fn from(s: MediaEncodingConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaEncodingConfiguration> for Any {
    fn from(s: &MediaEncodingConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MediaEncodingConfiguration {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> MediaEncodingType {
        self.inner.get("type").as_::<MediaEncodingType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &MediaEncodingType) {
        self.inner.set("type", value);
    }
}
