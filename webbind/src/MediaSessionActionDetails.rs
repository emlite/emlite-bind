use super::*;

/// The MediaSessionActionDetails dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaSessionActionDetails {
    inner: Any,
}

impl FromVal for MediaSessionActionDetails {
    fn from_val(v: &Any) -> Self {
        MediaSessionActionDetails { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaSessionActionDetails {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaSessionActionDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaSessionActionDetails {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaSessionActionDetails {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaSessionActionDetails> for Any {
    fn from(s: MediaSessionActionDetails) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaSessionActionDetails> for Any {
    fn from(s: &MediaSessionActionDetails) -> Any {
        s.inner.clone()
    }
}

impl MediaSessionActionDetails {
    /// Getter of the `action` attribute.
    pub fn action(&self) -> MediaSessionAction {
        self.inner.get("action").as_::<MediaSessionAction>()
    }

    /// Setter of the `action` attribute.
    pub fn set_action(&mut self, value: &MediaSessionAction) {
        self.inner.set("action", value);
    }
}
impl MediaSessionActionDetails {
    /// Getter of the `seekOffset` attribute.
    pub fn seek_offset(&self) -> f64 {
        self.inner.get("seekOffset").as_::<f64>()
    }

    /// Setter of the `seekOffset` attribute.
    pub fn set_seek_offset(&mut self, value: f64) {
        self.inner.set("seekOffset", value);
    }
}
impl MediaSessionActionDetails {
    /// Getter of the `seekTime` attribute.
    pub fn seek_time(&self) -> f64 {
        self.inner.get("seekTime").as_::<f64>()
    }

    /// Setter of the `seekTime` attribute.
    pub fn set_seek_time(&mut self, value: f64) {
        self.inner.set("seekTime", value);
    }
}
impl MediaSessionActionDetails {
    /// Getter of the `fastSeek` attribute.
    pub fn fast_seek(&self) -> bool {
        self.inner.get("fastSeek").as_::<bool>()
    }

    /// Setter of the `fastSeek` attribute.
    pub fn set_fast_seek(&mut self, value: bool) {
        self.inner.set("fastSeek", value);
    }
}
impl MediaSessionActionDetails {
    /// Getter of the `isActivating` attribute.
    pub fn is_activating(&self) -> bool {
        self.inner.get("isActivating").as_::<bool>()
    }

    /// Setter of the `isActivating` attribute.
    pub fn set_is_activating(&mut self, value: bool) {
        self.inner.set("isActivating", value);
    }
}
