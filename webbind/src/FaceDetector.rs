use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for DetectedFace {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DetectedFace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DetectedFace> for emlite::Val {
    fn from(s: DetectedFace) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
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
    pub fn landmarks(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("landmarks")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_landmarks(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("landmarks", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for FaceDetector {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FaceDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FaceDetector> for emlite::Val {
    fn from(s: FaceDetector) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FaceDetector {
    pub fn new0() -> FaceDetector {
        Self {
            inner: emlite::Val::global("FaceDetector")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(face_detector_options: jsbind::Any) -> FaceDetector {
        Self {
            inner: emlite::Val::global("FaceDetector")
                .new(&[face_detector_options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl FaceDetector {
    pub fn detect(&self, image: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("detect", &[image.into()])
            .as_::<jsbind::Promise>()
    }
}
