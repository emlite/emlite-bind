use super::*;

/// The FaceDetectorOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FaceDetectorOptions {
    inner: Any,
}

impl FromVal for FaceDetectorOptions {
    fn from_val(v: &Any) -> Self {
        FaceDetectorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FaceDetectorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FaceDetectorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FaceDetectorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FaceDetectorOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FaceDetectorOptions> for Any {
    fn from(s: FaceDetectorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FaceDetectorOptions> for Any {
    fn from(s: &FaceDetectorOptions) -> Any {
        s.inner.clone()
    }
}

impl FaceDetectorOptions {
    /// Getter of the `maxDetectedFaces` attribute.
    pub fn max_detected_faces(&self) -> u16 {
        self.inner.get("maxDetectedFaces").as_::<u16>()
    }

    /// Setter of the `maxDetectedFaces` attribute.
    pub fn set_max_detected_faces(&mut self, value: u16) {
        self.inner.set("maxDetectedFaces", value);
    }
}
impl FaceDetectorOptions {
    /// Getter of the `fastMode` attribute.
    pub fn fast_mode(&self) -> bool {
        self.inner.get("fastMode").as_::<bool>()
    }

    /// Setter of the `fastMode` attribute.
    pub fn set_fast_mode(&mut self, value: bool) {
        self.inner.set("fastMode", value);
    }
}
