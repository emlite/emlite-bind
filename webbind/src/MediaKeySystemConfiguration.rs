use super::*;

/// The MediaKeySystemConfiguration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeySystemConfiguration {
    inner: Any,
}

impl FromVal for MediaKeySystemConfiguration {
    fn from_val(v: &Any) -> Self {
        MediaKeySystemConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaKeySystemConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaKeySystemConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaKeySystemConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaKeySystemConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaKeySystemConfiguration> for Any {
    fn from(s: MediaKeySystemConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaKeySystemConfiguration> for Any {
    fn from(s: &MediaKeySystemConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MediaKeySystemConfiguration {
    /// Getter of the `label` attribute.
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl MediaKeySystemConfiguration {
    /// Getter of the `initDataTypes` attribute.
    pub fn init_data_types(&self) -> TypedArray<JsString> {
        self.inner
            .get("initDataTypes")
            .as_::<TypedArray<JsString>>()
    }

    /// Setter of the `initDataTypes` attribute.
    pub fn set_init_data_types(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("initDataTypes", value);
    }
}
impl MediaKeySystemConfiguration {
    /// Getter of the `audioCapabilities` attribute.
    pub fn audio_capabilities(&self) -> TypedArray<MediaKeySystemMediaCapability> {
        self.inner
            .get("audioCapabilities")
            .as_::<TypedArray<MediaKeySystemMediaCapability>>()
    }

    /// Setter of the `audioCapabilities` attribute.
    pub fn set_audio_capabilities(&mut self, value: &TypedArray<MediaKeySystemMediaCapability>) {
        self.inner.set("audioCapabilities", value);
    }
}
impl MediaKeySystemConfiguration {
    /// Getter of the `videoCapabilities` attribute.
    pub fn video_capabilities(&self) -> TypedArray<MediaKeySystemMediaCapability> {
        self.inner
            .get("videoCapabilities")
            .as_::<TypedArray<MediaKeySystemMediaCapability>>()
    }

    /// Setter of the `videoCapabilities` attribute.
    pub fn set_video_capabilities(&mut self, value: &TypedArray<MediaKeySystemMediaCapability>) {
        self.inner.set("videoCapabilities", value);
    }
}
impl MediaKeySystemConfiguration {
    /// Getter of the `distinctiveIdentifier` attribute.
    pub fn distinctive_identifier(&self) -> MediaKeysRequirement {
        self.inner
            .get("distinctiveIdentifier")
            .as_::<MediaKeysRequirement>()
    }

    /// Setter of the `distinctiveIdentifier` attribute.
    pub fn set_distinctive_identifier(&mut self, value: &MediaKeysRequirement) {
        self.inner.set("distinctiveIdentifier", value);
    }
}
impl MediaKeySystemConfiguration {
    /// Getter of the `persistentState` attribute.
    pub fn persistent_state(&self) -> MediaKeysRequirement {
        self.inner
            .get("persistentState")
            .as_::<MediaKeysRequirement>()
    }

    /// Setter of the `persistentState` attribute.
    pub fn set_persistent_state(&mut self, value: &MediaKeysRequirement) {
        self.inner.set("persistentState", value);
    }
}
impl MediaKeySystemConfiguration {
    /// Getter of the `sessionTypes` attribute.
    pub fn session_types(&self) -> TypedArray<JsString> {
        self.inner.get("sessionTypes").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `sessionTypes` attribute.
    pub fn set_session_types(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("sessionTypes", value);
    }
}
