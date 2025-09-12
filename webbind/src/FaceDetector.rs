use super::*;

/// The FaceDetector class.
/// [`FaceDetector`](https://developer.mozilla.org/en-US/docs/Web/API/FaceDetector)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FaceDetector {
    inner: Any,
}

impl FromVal for FaceDetector {
    fn from_val(v: &Any) -> Self {
        FaceDetector {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FaceDetector {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FaceDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FaceDetector {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FaceDetector {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FaceDetector> for Any {
    fn from(s: FaceDetector) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FaceDetector> for Any {
    fn from(s: &FaceDetector) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FaceDetector);

impl FaceDetector {
    /// The `new FaceDetector(..)` constructor, creating a new FaceDetector instance
    pub fn new() -> FaceDetector {
        Self {
            inner: Any::global("FaceDetector").new(&[]).as_::<Any>(),
        }
    }
}

impl FaceDetector {
    /// The `new FaceDetector(..)` constructor, creating a new FaceDetector instance
    pub fn new_with_face_detector_options(
        face_detector_options: &FaceDetectorOptions,
    ) -> FaceDetector {
        Self {
            inner: Any::global("FaceDetector")
                .new(&[face_detector_options.into()])
                .as_::<Any>(),
        }
    }
}

impl FaceDetector {
    /// The detect method.
    /// [`FaceDetector.detect`](https://developer.mozilla.org/en-US/docs/Web/API/FaceDetector/detect)
    pub fn detect(&self, image: &Any) -> Promise<TypedArray<DetectedFace>> {
        self.inner
            .call("detect", &[image.into()])
            .as_::<Promise<TypedArray<DetectedFace>>>()
    }
}
