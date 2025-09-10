use super::*;

/// The MediaTrackCapabilities dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaTrackCapabilities {
    inner: Any,
}

impl FromVal for MediaTrackCapabilities {
    fn from_val(v: &Any) -> Self {
        MediaTrackCapabilities { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaTrackCapabilities {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaTrackCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaTrackCapabilities {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaTrackCapabilities {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaTrackCapabilities> for Any {
    fn from(s: MediaTrackCapabilities) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaTrackCapabilities> for Any {
    fn from(s: &MediaTrackCapabilities) -> Any {
        s.inner.clone()
    }
}

impl MediaTrackCapabilities {
    /// Getter of the `displaySurface` attribute.
    pub fn display_surface(&self) -> JsString {
        self.inner.get("displaySurface").as_::<JsString>()
    }

    /// Setter of the `displaySurface` attribute.
    pub fn set_display_surface(&mut self, value: &JsString) {
        self.inner.set("displaySurface", value);
    }
}
impl MediaTrackCapabilities {
    /// Getter of the `logicalSurface` attribute.
    pub fn logical_surface(&self) -> bool {
        self.inner.get("logicalSurface").as_::<bool>()
    }

    /// Setter of the `logicalSurface` attribute.
    pub fn set_logical_surface(&mut self, value: bool) {
        self.inner.set("logicalSurface", value);
    }
}
impl MediaTrackCapabilities {
    /// Getter of the `cursor` attribute.
    pub fn cursor(&self) -> TypedArray<JsString> {
        self.inner.get("cursor").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `cursor` attribute.
    pub fn set_cursor(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("cursor", value);
    }
}
