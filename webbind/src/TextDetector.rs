use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DetectedText {
    inner: Any,
}
impl FromVal for DetectedText {
    fn from_val(v: &Any) -> Self {
        DetectedText { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DetectedText {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DetectedText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DetectedText {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DetectedText {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DetectedText> for Any {
    fn from(s: DetectedText) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DetectedText> for Any {
    fn from(s: &DetectedText) -> Any {
        s.inner.clone()
    }
}

impl DetectedText {
    pub fn bounding_box(&self) -> DOMRectReadOnly {
        self.inner.get("boundingBox").as_::<DOMRectReadOnly>()
    }

    pub fn set_bounding_box(&mut self, value: &DOMRectReadOnly) {
        self.inner.set("boundingBox", value);
    }
}
impl DetectedText {
    pub fn raw_value(&self) -> String {
        self.inner.get("rawValue").as_::<String>()
    }

    pub fn set_raw_value(&mut self, value: &str) {
        self.inner.set("rawValue", value);
    }
}
impl DetectedText {
    pub fn corner_points(&self) -> Sequence<Any> {
        self.inner.get("cornerPoints").as_::<Sequence<Any>>()
    }

    pub fn set_corner_points(&mut self, value: &Sequence<Any>) {
        self.inner.set("cornerPoints", value);
    }
}
/// The TextDetector class.
/// [`TextDetector`](https://developer.mozilla.org/en-US/docs/Web/API/TextDetector)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextDetector {
    inner: Any,
}
impl FromVal for TextDetector {
    fn from_val(v: &Any) -> Self {
        TextDetector {
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
impl core::ops::Deref for TextDetector {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TextDetector {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextDetector {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextDetector> for Any {
    fn from(s: TextDetector) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextDetector> for Any {
    fn from(s: &TextDetector) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextDetector);

impl TextDetector {
    /// The `new TextDetector(..)` constructor, creating a new TextDetector instance
    pub fn new() -> TextDetector {
        Self {
            inner: Any::global("TextDetector").new(&[]).as_::<Any>(),
        }
    }
}
impl TextDetector {
    /// The detect method.
    /// [`TextDetector.detect`](https://developer.mozilla.org/en-US/docs/Web/API/TextDetector/detect)
    pub fn detect(&self, image: &Any) -> Promise<Sequence<DetectedText>> {
        self.inner
            .call("detect", &[image.into()])
            .as_::<Promise<Sequence<DetectedText>>>()
    }
}
