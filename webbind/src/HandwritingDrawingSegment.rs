use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingDrawingSegment {
    inner: Any,
}
impl FromVal for HandwritingDrawingSegment {
    fn from_val(v: &Any) -> Self {
        HandwritingDrawingSegment { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HandwritingDrawingSegment {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingDrawingSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HandwritingDrawingSegment {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HandwritingDrawingSegment {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HandwritingDrawingSegment> for Any {
    fn from(s: HandwritingDrawingSegment) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HandwritingDrawingSegment> for Any {
    fn from(s: &HandwritingDrawingSegment) -> Any {
        s.inner.clone()
    }
}

impl HandwritingDrawingSegment {
    pub fn stroke_index(&self) -> u32 {
        self.inner.get("strokeIndex").as_::<u32>()
    }

    pub fn set_stroke_index(&mut self, value: u32) {
        self.inner.set("strokeIndex", value);
    }
}
impl HandwritingDrawingSegment {
    pub fn begin_point_index(&self) -> u32 {
        self.inner.get("beginPointIndex").as_::<u32>()
    }

    pub fn set_begin_point_index(&mut self, value: u32) {
        self.inner.set("beginPointIndex", value);
    }
}
impl HandwritingDrawingSegment {
    pub fn end_point_index(&self) -> u32 {
        self.inner.get("endPointIndex").as_::<u32>()
    }

    pub fn set_end_point_index(&mut self, value: u32) {
        self.inner.set("endPointIndex", value);
    }
}
