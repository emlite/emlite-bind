use super::*;

/// The RTCMediaSourceStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCMediaSourceStats {
    inner: Any,
}

impl FromVal for RTCMediaSourceStats {
    fn from_val(v: &Any) -> Self {
        RTCMediaSourceStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCMediaSourceStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCMediaSourceStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCMediaSourceStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCMediaSourceStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCMediaSourceStats> for Any {
    fn from(s: RTCMediaSourceStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCMediaSourceStats> for Any {
    fn from(s: &RTCMediaSourceStats) -> Any {
        s.inner.clone()
    }
}

impl RTCMediaSourceStats {
    /// Getter of the `trackIdentifier` attribute.
    pub fn track_identifier(&self) -> JsString {
        self.inner.get("trackIdentifier").as_::<JsString>()
    }

    /// Setter of the `trackIdentifier` attribute.
    pub fn set_track_identifier(&mut self, value: &JsString) {
        self.inner.set("trackIdentifier", value);
    }
}
impl RTCMediaSourceStats {
    /// Getter of the `kind` attribute.
    pub fn kind(&self) -> JsString {
        self.inner.get("kind").as_::<JsString>()
    }

    /// Setter of the `kind` attribute.
    pub fn set_kind(&mut self, value: &JsString) {
        self.inner.set("kind", value);
    }
}
