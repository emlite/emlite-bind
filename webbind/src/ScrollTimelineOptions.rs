use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScrollTimelineOptions {
    inner: Any,
}
impl FromVal for ScrollTimelineOptions {
    fn from_val(v: &Any) -> Self {
        ScrollTimelineOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ScrollTimelineOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ScrollTimelineOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ScrollTimelineOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ScrollTimelineOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ScrollTimelineOptions> for Any {
    fn from(s: ScrollTimelineOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ScrollTimelineOptions> for Any {
    fn from(s: &ScrollTimelineOptions) -> Any {
        s.inner.clone()
    }
}

impl ScrollTimelineOptions {
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }

    pub fn set_source(&mut self, value: &Element) {
        self.inner.set("source", value);
    }
}
impl ScrollTimelineOptions {
    pub fn axis(&self) -> ScrollAxis {
        self.inner.get("axis").as_::<ScrollAxis>()
    }

    pub fn set_axis(&mut self, value: &ScrollAxis) {
        self.inner.set("axis", value);
    }
}
