use super::*;

/// The PointerTimelineOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PointerTimelineOptions {
    inner: Any,
}

impl FromVal for PointerTimelineOptions {
    fn from_val(v: &Any) -> Self {
        PointerTimelineOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PointerTimelineOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PointerTimelineOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PointerTimelineOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PointerTimelineOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PointerTimelineOptions> for Any {
    fn from(s: PointerTimelineOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PointerTimelineOptions> for Any {
    fn from(s: &PointerTimelineOptions) -> Any {
        s.inner.clone()
    }
}

impl PointerTimelineOptions {
    /// Getter of the `source` attribute.
    pub fn source(&self) -> Element {
        self.inner.get("source").as_::<Element>()
    }

    /// Setter of the `source` attribute.
    pub fn set_source(&mut self, value: &Element) {
        self.inner.set("source", value);
    }
}
impl PointerTimelineOptions {
    /// Getter of the `axis` attribute.
    pub fn axis(&self) -> PointerAxis {
        self.inner.get("axis").as_::<PointerAxis>()
    }

    /// Setter of the `axis` attribute.
    pub fn set_axis(&mut self, value: &PointerAxis) {
        self.inner.set("axis", value);
    }
}
