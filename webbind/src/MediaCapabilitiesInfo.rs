use super::*;

/// The MediaCapabilitiesInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaCapabilitiesInfo {
    inner: Any,
}

impl FromVal for MediaCapabilitiesInfo {
    fn from_val(v: &Any) -> Self {
        MediaCapabilitiesInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaCapabilitiesInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaCapabilitiesInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaCapabilitiesInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaCapabilitiesInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaCapabilitiesInfo> for Any {
    fn from(s: MediaCapabilitiesInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaCapabilitiesInfo> for Any {
    fn from(s: &MediaCapabilitiesInfo) -> Any {
        s.inner.clone()
    }
}

impl MediaCapabilitiesInfo {
    /// Getter of the `supported` attribute.
    pub fn supported(&self) -> bool {
        self.inner.get("supported").as_::<bool>()
    }

    /// Setter of the `supported` attribute.
    pub fn set_supported(&mut self, value: bool) {
        self.inner.set("supported", value);
    }
}
impl MediaCapabilitiesInfo {
    /// Getter of the `smooth` attribute.
    pub fn smooth(&self) -> bool {
        self.inner.get("smooth").as_::<bool>()
    }

    /// Setter of the `smooth` attribute.
    pub fn set_smooth(&mut self, value: bool) {
        self.inner.set("smooth", value);
    }
}
impl MediaCapabilitiesInfo {
    /// Getter of the `powerEfficient` attribute.
    pub fn power_efficient(&self) -> bool {
        self.inner.get("powerEfficient").as_::<bool>()
    }

    /// Setter of the `powerEfficient` attribute.
    pub fn set_power_efficient(&mut self, value: bool) {
        self.inner.set("powerEfficient", value);
    }
}
