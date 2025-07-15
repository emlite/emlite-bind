use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DetectedFace {
    inner: emlite::Val,
}
impl FromVal for DetectedFace {
    fn from_val(v: &emlite::Val) -> Self {
        DetectedFace { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DetectedFace {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DetectedFace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DetectedFace {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DetectedFace {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DetectedFace> for emlite::Val {
    fn from(s: DetectedFace) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DetectedFace {
    pub fn bounding_box(&self) -> DOMRectReadOnly {
        self.inner.get("boundingBox").as_::<DOMRectReadOnly>()
    }

    pub fn set_bounding_box(&mut self, value: DOMRectReadOnly) {
        self.inner.set("boundingBox", value);
    }
}
impl DetectedFace {
    pub fn landmarks(&self) -> Sequence<Any> {
        self.inner.get("landmarks").as_::<Sequence<Any>>()
    }

    pub fn set_landmarks(&mut self, value: Sequence<Any>) {
        self.inner.set("landmarks", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FaceDetector {
    inner: emlite::Val,
}
impl FromVal for FaceDetector {
    fn from_val(v: &emlite::Val) -> Self {
        FaceDetector {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FaceDetector {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FaceDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FaceDetector {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FaceDetector {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FaceDetector> for emlite::Val {
    fn from(s: FaceDetector) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FaceDetector);

impl FaceDetector {
    pub fn new0() -> FaceDetector {
        Self {
            inner: emlite::Val::global("FaceDetector")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(face_detector_options: Any) -> FaceDetector {
        Self {
            inner: emlite::Val::global("FaceDetector")
                .new(&[face_detector_options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl FaceDetector {
    pub fn detect(&self, image: Any) -> Promise {
        self.inner.call("detect", &[image.into()]).as_::<Promise>()
    }
}
