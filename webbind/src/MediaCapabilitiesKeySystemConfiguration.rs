use super::*;




/// The MediaCapabilitiesKeySystemConfiguration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaCapabilitiesKeySystemConfiguration {
    inner: Any,
}

impl FromVal for MediaCapabilitiesKeySystemConfiguration {
    fn from_val(v: &Any) -> Self {
        MediaCapabilitiesKeySystemConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaCapabilitiesKeySystemConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaCapabilitiesKeySystemConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaCapabilitiesKeySystemConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaCapabilitiesKeySystemConfiguration {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MediaCapabilitiesKeySystemConfiguration> for Any {
    fn from(s: MediaCapabilitiesKeySystemConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaCapabilitiesKeySystemConfiguration> for Any {
    fn from(s: &MediaCapabilitiesKeySystemConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MediaCapabilitiesKeySystemConfiguration {
    /// Getter of the `keySystem` attribute.
    pub fn key_system(&self) -> JsString {
        self.inner.get("keySystem").as_::<JsString>()
    }

    /// Setter of the `keySystem` attribute.
    pub fn set_key_system(&mut self, value: &JsString) {
        self.inner.set("keySystem", value);
    }
}
impl MediaCapabilitiesKeySystemConfiguration {
    /// Getter of the `initDataType` attribute.
    pub fn init_data_type(&self) -> JsString {
        self.inner.get("initDataType").as_::<JsString>()
    }

    /// Setter of the `initDataType` attribute.
    pub fn set_init_data_type(&mut self, value: &JsString) {
        self.inner.set("initDataType", value);
    }
}
impl MediaCapabilitiesKeySystemConfiguration {
    /// Getter of the `distinctiveIdentifier` attribute.
    pub fn distinctive_identifier(&self) -> MediaKeysRequirement {
        self.inner.get("distinctiveIdentifier").as_::<MediaKeysRequirement>()
    }

    /// Setter of the `distinctiveIdentifier` attribute.
    pub fn set_distinctive_identifier(&mut self, value: &MediaKeysRequirement) {
        self.inner.set("distinctiveIdentifier", value);
    }
}
impl MediaCapabilitiesKeySystemConfiguration {
    /// Getter of the `persistentState` attribute.
    pub fn persistent_state(&self) -> MediaKeysRequirement {
        self.inner.get("persistentState").as_::<MediaKeysRequirement>()
    }

    /// Setter of the `persistentState` attribute.
    pub fn set_persistent_state(&mut self, value: &MediaKeysRequirement) {
        self.inner.set("persistentState", value);
    }
}
impl MediaCapabilitiesKeySystemConfiguration {
    /// Getter of the `sessionTypes` attribute.
    pub fn session_types(&self) -> TypedArray<JsString> {
        self.inner.get("sessionTypes").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `sessionTypes` attribute.
    pub fn set_session_types(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("sessionTypes", value);
    }
}
impl MediaCapabilitiesKeySystemConfiguration {
    /// Getter of the `audio` attribute.
    pub fn audio(&self) -> KeySystemTrackConfiguration {
        self.inner.get("audio").as_::<KeySystemTrackConfiguration>()
    }

    /// Setter of the `audio` attribute.
    pub fn set_audio(&mut self, value: &KeySystemTrackConfiguration) {
        self.inner.set("audio", value);
    }
}
impl MediaCapabilitiesKeySystemConfiguration {
    /// Getter of the `video` attribute.
    pub fn video(&self) -> KeySystemTrackConfiguration {
        self.inner.get("video").as_::<KeySystemTrackConfiguration>()
    }

    /// Setter of the `video` attribute.
    pub fn set_video(&mut self, value: &KeySystemTrackConfiguration) {
        self.inner.set("video", value);
    }
}
