use super::*;

/// The TimelineRangeOffset dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TimelineRangeOffset {
    inner: Any,
}

impl FromVal for TimelineRangeOffset {
    fn from_val(v: &Any) -> Self {
        TimelineRangeOffset { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TimelineRangeOffset {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TimelineRangeOffset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TimelineRangeOffset {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TimelineRangeOffset {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TimelineRangeOffset> for Any {
    fn from(s: TimelineRangeOffset) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TimelineRangeOffset> for Any {
    fn from(s: &TimelineRangeOffset) -> Any {
        s.inner.clone()
    }
}

impl TimelineRangeOffset {
    /// Getter of the `rangeName` attribute.
    pub fn range_name(&self) -> JsString {
        self.inner.get("rangeName").as_::<JsString>()
    }

    /// Setter of the `rangeName` attribute.
    pub fn set_range_name(&mut self, value: &JsString) {
        self.inner.set("rangeName", value);
    }
}
impl TimelineRangeOffset {
    /// Getter of the `offset` attribute.
    pub fn offset(&self) -> CSSNumericValue {
        self.inner.get("offset").as_::<CSSNumericValue>()
    }

    /// Setter of the `offset` attribute.
    pub fn set_offset(&mut self, value: &CSSNumericValue) {
        self.inner.set("offset", value);
    }
}
