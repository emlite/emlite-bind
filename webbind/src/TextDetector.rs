use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DetectedText {
    inner: emlite::Val,
}
impl FromVal for DetectedText {
    fn from_val(v: &emlite::Val) -> Self {
        DetectedText { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DetectedText {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DetectedText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DetectedText {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DetectedText {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DetectedText> for emlite::Val {
    fn from(s: DetectedText) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DetectedText {
    pub fn bounding_box(&self) -> DOMRectReadOnly {
        self.inner.get("boundingBox").as_::<DOMRectReadOnly>()
    }

    pub fn set_bounding_box(&mut self, value: DOMRectReadOnly) {
        self.inner.set("boundingBox", value);
    }
}
impl DetectedText {
    pub fn raw_value(&self) -> jsbind::DOMString {
        self.inner.get("rawValue").as_::<jsbind::DOMString>()
    }

    pub fn set_raw_value(&mut self, value: jsbind::DOMString) {
        self.inner.set("rawValue", value);
    }
}
impl DetectedText {
    pub fn corner_points(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("cornerPoints")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_corner_points(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("cornerPoints", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextDetector {
    inner: emlite::Val,
}
impl FromVal for TextDetector {
    fn from_val(v: &emlite::Val) -> Self {
        TextDetector {
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
impl core::ops::Deref for TextDetector {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for TextDetector {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for TextDetector {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<TextDetector> for emlite::Val {
    fn from(s: TextDetector) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(TextDetector);

impl TextDetector {
    pub fn new() -> TextDetector {
        Self {
            inner: emlite::Val::global("TextDetector")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl TextDetector {
    pub fn detect(&self, image: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("detect", &[image.into()])
            .as_::<jsbind::Promise>()
    }
}
