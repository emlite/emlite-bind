use super::*;

/// The VideoColorSpaceInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoColorSpaceInit {
    inner: Any,
}

impl FromVal for VideoColorSpaceInit {
    fn from_val(v: &Any) -> Self {
        VideoColorSpaceInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoColorSpaceInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoColorSpaceInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoColorSpaceInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoColorSpaceInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<VideoColorSpaceInit> for Any {
    fn from(s: VideoColorSpaceInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoColorSpaceInit> for Any {
    fn from(s: &VideoColorSpaceInit) -> Any {
        s.inner.clone()
    }
}

impl VideoColorSpaceInit {
    /// Getter of the `primaries` attribute.
    pub fn primaries(&self) -> VideoColorPrimaries {
        self.inner.get("primaries").as_::<VideoColorPrimaries>()
    }

    /// Setter of the `primaries` attribute.
    pub fn set_primaries(&mut self, value: &VideoColorPrimaries) {
        self.inner.set("primaries", value);
    }
}
impl VideoColorSpaceInit {
    /// Getter of the `transfer` attribute.
    pub fn transfer(&self) -> VideoTransferCharacteristics {
        self.inner
            .get("transfer")
            .as_::<VideoTransferCharacteristics>()
    }

    /// Setter of the `transfer` attribute.
    pub fn set_transfer(&mut self, value: &VideoTransferCharacteristics) {
        self.inner.set("transfer", value);
    }
}
impl VideoColorSpaceInit {
    /// Getter of the `matrix` attribute.
    pub fn matrix(&self) -> VideoMatrixCoefficients {
        self.inner.get("matrix").as_::<VideoMatrixCoefficients>()
    }

    /// Setter of the `matrix` attribute.
    pub fn set_matrix(&mut self, value: &VideoMatrixCoefficients) {
        self.inner.set("matrix", value);
    }
}
impl VideoColorSpaceInit {
    /// Getter of the `fullRange` attribute.
    pub fn full_range(&self) -> bool {
        self.inner.get("fullRange").as_::<bool>()
    }

    /// Setter of the `fullRange` attribute.
    pub fn set_full_range(&mut self, value: bool) {
        self.inner.set("fullRange", value);
    }
}
