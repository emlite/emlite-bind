use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DetectedFace {
    inner: Any,
}
impl FromVal for DetectedFace {
    fn from_val(v: &Any) -> Self {
        DetectedFace { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DetectedFace {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DetectedFace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DetectedFace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DetectedFace {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DetectedFace> for Any {
    fn from(s: DetectedFace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DetectedFace> for Any {
    fn from(s: &DetectedFace) -> Any {
        s.inner.clone()
    }
}

impl DetectedFace {
    pub fn bounding_box(&self) -> DOMRectReadOnly {
        self.inner.get("boundingBox").as_::<DOMRectReadOnly>()
    }

    pub fn set_bounding_box(&mut self, value: &DOMRectReadOnly) {
        self.inner.set("boundingBox", value);
    }
}
impl DetectedFace {
    pub fn landmarks(&self) -> TypedArray<Any> {
        self.inner.get("landmarks").as_::<TypedArray<Any>>()
    }

    pub fn set_landmarks(&mut self, value: &TypedArray<Any>) {
        self.inner.set("landmarks", value);
    }
}
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
    pub fn new0() -> FaceDetector {
        Self {
            inner: Any::global("FaceDetector").new(&[]).as_::<Any>(),
        }
    }

    /// The `new FaceDetector(..)` constructor, creating a new FaceDetector instance
    pub fn new1(face_detector_options: &Any) -> FaceDetector {
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
