use super::*;




/// The ViewTimelineOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ViewTimelineOptions {
    inner: Any,
}

impl FromVal for ViewTimelineOptions {
    fn from_val(v: &Any) -> Self {
        ViewTimelineOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ViewTimelineOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ViewTimelineOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ViewTimelineOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ViewTimelineOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ViewTimelineOptions> for Any {
    fn from(s: ViewTimelineOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ViewTimelineOptions> for Any {
    fn from(s: &ViewTimelineOptions) -> Any {
        s.inner.clone()
    }
}

impl ViewTimelineOptions {
    /// Getter of the `subject` attribute.
    pub fn subject(&self) -> Element {
        self.inner.get("subject").as_::<Element>()
    }

    /// Setter of the `subject` attribute.
    pub fn set_subject(&mut self, value: &Element) {
        self.inner.set("subject", value);
    }
}
impl ViewTimelineOptions {
    /// Getter of the `axis` attribute.
    pub fn axis(&self) -> ScrollAxis {
        self.inner.get("axis").as_::<ScrollAxis>()
    }

    /// Setter of the `axis` attribute.
    pub fn set_axis(&mut self, value: &ScrollAxis) {
        self.inner.set("axis", value);
    }
}
impl ViewTimelineOptions {
    /// Getter of the `inset` attribute.
    pub fn inset(&self) -> Any {
        self.inner.get("inset").as_::<Any>()
    }

    /// Setter of the `inset` attribute.
    pub fn set_inset(&mut self, value: &Any) {
        self.inner.set("inset", value);
    }
}
