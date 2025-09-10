use super::*;

/// The RTCVideoSourceStats dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCVideoSourceStats {
    inner: Any,
}

impl FromVal for RTCVideoSourceStats {
    fn from_val(v: &Any) -> Self {
        RTCVideoSourceStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCVideoSourceStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCVideoSourceStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCVideoSourceStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCVideoSourceStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCVideoSourceStats> for Any {
    fn from(s: RTCVideoSourceStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCVideoSourceStats> for Any {
    fn from(s: &RTCVideoSourceStats) -> Any {
        s.inner.clone()
    }
}

impl RTCVideoSourceStats {
    /// Getter of the `width` attribute.
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }

    /// Setter of the `width` attribute.
    pub fn set_width(&mut self, value: u32) {
        self.inner.set("width", value);
    }
}
impl RTCVideoSourceStats {
    /// Getter of the `height` attribute.
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }

    /// Setter of the `height` attribute.
    pub fn set_height(&mut self, value: u32) {
        self.inner.set("height", value);
    }
}
impl RTCVideoSourceStats {
    /// Getter of the `frames` attribute.
    pub fn frames(&self) -> u32 {
        self.inner.get("frames").as_::<u32>()
    }

    /// Setter of the `frames` attribute.
    pub fn set_frames(&mut self, value: u32) {
        self.inner.set("frames", value);
    }
}
impl RTCVideoSourceStats {
    /// Getter of the `framesPerSecond` attribute.
    pub fn frames_per_second(&self) -> f64 {
        self.inner.get("framesPerSecond").as_::<f64>()
    }

    /// Setter of the `framesPerSecond` attribute.
    pub fn set_frames_per_second(&mut self, value: f64) {
        self.inner.set("framesPerSecond", value);
    }
}
