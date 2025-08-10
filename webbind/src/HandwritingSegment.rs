use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingSegment {
    inner: Any,
}
impl FromVal for HandwritingSegment {
    fn from_val(v: &Any) -> Self {
        HandwritingSegment { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HandwritingSegment {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HandwritingSegment {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HandwritingSegment {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HandwritingSegment> for Any {
    fn from(s: HandwritingSegment) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HandwritingSegment> for Any {
    fn from(s: &HandwritingSegment) -> Any {
        s.inner.clone()
    }
}

impl HandwritingSegment {
    pub fn grapheme(&self) -> JsString {
        self.inner.get("grapheme").as_::<JsString>()
    }

    pub fn set_grapheme(&mut self, value: &JsString) {
        self.inner.set("grapheme", value);
    }
}
impl HandwritingSegment {
    pub fn begin_index(&self) -> u32 {
        self.inner.get("beginIndex").as_::<u32>()
    }

    pub fn set_begin_index(&mut self, value: u32) {
        self.inner.set("beginIndex", value);
    }
}
impl HandwritingSegment {
    pub fn end_index(&self) -> u32 {
        self.inner.get("endIndex").as_::<u32>()
    }

    pub fn set_end_index(&mut self, value: u32) {
        self.inner.set("endIndex", value);
    }
}
impl HandwritingSegment {
    pub fn drawing_segments(&self) -> TypedArray<HandwritingDrawingSegment> {
        self.inner
            .get("drawingSegments")
            .as_::<TypedArray<HandwritingDrawingSegment>>()
    }

    pub fn set_drawing_segments(&mut self, value: &TypedArray<HandwritingDrawingSegment>) {
        self.inner.set("drawingSegments", value);
    }
}
