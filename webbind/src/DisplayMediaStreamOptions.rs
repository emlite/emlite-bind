use super::*;

/// The DisplayMediaStreamOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DisplayMediaStreamOptions {
    inner: Any,
}

impl FromVal for DisplayMediaStreamOptions {
    fn from_val(v: &Any) -> Self {
        DisplayMediaStreamOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DisplayMediaStreamOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DisplayMediaStreamOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DisplayMediaStreamOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DisplayMediaStreamOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DisplayMediaStreamOptions> for Any {
    fn from(s: DisplayMediaStreamOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DisplayMediaStreamOptions> for Any {
    fn from(s: &DisplayMediaStreamOptions) -> Any {
        s.inner.clone()
    }
}

impl DisplayMediaStreamOptions {
    /// Getter of the `video` attribute.
    pub fn video(&self) -> Any {
        self.inner.get("video").as_::<Any>()
    }

    /// Setter of the `video` attribute.
    pub fn set_video(&mut self, value: &Any) {
        self.inner.set("video", value);
    }
}
impl DisplayMediaStreamOptions {
    /// Getter of the `audio` attribute.
    pub fn audio(&self) -> Any {
        self.inner.get("audio").as_::<Any>()
    }

    /// Setter of the `audio` attribute.
    pub fn set_audio(&mut self, value: &Any) {
        self.inner.set("audio", value);
    }
}
impl DisplayMediaStreamOptions {
    /// Getter of the `controller` attribute.
    pub fn controller(&self) -> CaptureController {
        self.inner.get("controller").as_::<CaptureController>()
    }

    /// Setter of the `controller` attribute.
    pub fn set_controller(&mut self, value: &CaptureController) {
        self.inner.set("controller", value);
    }
}
impl DisplayMediaStreamOptions {
    /// Getter of the `selfBrowserSurface` attribute.
    pub fn self_browser_surface(&self) -> SelfCapturePreferenceEnum {
        self.inner
            .get("selfBrowserSurface")
            .as_::<SelfCapturePreferenceEnum>()
    }

    /// Setter of the `selfBrowserSurface` attribute.
    pub fn set_self_browser_surface(&mut self, value: &SelfCapturePreferenceEnum) {
        self.inner.set("selfBrowserSurface", value);
    }
}
impl DisplayMediaStreamOptions {
    /// Getter of the `systemAudio` attribute.
    pub fn system_audio(&self) -> SystemAudioPreferenceEnum {
        self.inner
            .get("systemAudio")
            .as_::<SystemAudioPreferenceEnum>()
    }

    /// Setter of the `systemAudio` attribute.
    pub fn set_system_audio(&mut self, value: &SystemAudioPreferenceEnum) {
        self.inner.set("systemAudio", value);
    }
}
impl DisplayMediaStreamOptions {
    /// Getter of the `windowAudio` attribute.
    pub fn window_audio(&self) -> WindowAudioPreferenceEnum {
        self.inner
            .get("windowAudio")
            .as_::<WindowAudioPreferenceEnum>()
    }

    /// Setter of the `windowAudio` attribute.
    pub fn set_window_audio(&mut self, value: &WindowAudioPreferenceEnum) {
        self.inner.set("windowAudio", value);
    }
}
impl DisplayMediaStreamOptions {
    /// Getter of the `surfaceSwitching` attribute.
    pub fn surface_switching(&self) -> SurfaceSwitchingPreferenceEnum {
        self.inner
            .get("surfaceSwitching")
            .as_::<SurfaceSwitchingPreferenceEnum>()
    }

    /// Setter of the `surfaceSwitching` attribute.
    pub fn set_surface_switching(&mut self, value: &SurfaceSwitchingPreferenceEnum) {
        self.inner.set("surfaceSwitching", value);
    }
}
impl DisplayMediaStreamOptions {
    /// Getter of the `monitorTypeSurfaces` attribute.
    pub fn monitor_type_surfaces(&self) -> MonitorTypeSurfacesEnum {
        self.inner
            .get("monitorTypeSurfaces")
            .as_::<MonitorTypeSurfacesEnum>()
    }

    /// Setter of the `monitorTypeSurfaces` attribute.
    pub fn set_monitor_type_surfaces(&mut self, value: &MonitorTypeSurfacesEnum) {
        self.inner.set("monitorTypeSurfaces", value);
    }
}
