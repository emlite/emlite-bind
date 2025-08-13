use super::*;




/// The ScrollIntoViewOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScrollIntoViewOptions {
    inner: Any,
}

impl FromVal for ScrollIntoViewOptions {
    fn from_val(v: &Any) -> Self {
        ScrollIntoViewOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ScrollIntoViewOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ScrollIntoViewOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ScrollIntoViewOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ScrollIntoViewOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<ScrollIntoViewOptions> for Any {
    fn from(s: ScrollIntoViewOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ScrollIntoViewOptions> for Any {
    fn from(s: &ScrollIntoViewOptions) -> Any {
        s.inner.clone()
    }
}

impl ScrollIntoViewOptions {
    /// Getter of the `block` attribute.
    pub fn block(&self) -> ScrollLogicalPosition {
        self.inner.get("block").as_::<ScrollLogicalPosition>()
    }

    /// Setter of the `block` attribute.
    pub fn set_block(&mut self, value: &ScrollLogicalPosition) {
        self.inner.set("block", value);
    }
}
impl ScrollIntoViewOptions {
    /// Getter of the `inline` attribute.
    pub fn inline(&self) -> ScrollLogicalPosition {
        self.inner.get("inline").as_::<ScrollLogicalPosition>()
    }

    /// Setter of the `inline` attribute.
    pub fn set_inline(&mut self, value: &ScrollLogicalPosition) {
        self.inner.set("inline", value);
    }
}
impl ScrollIntoViewOptions {
    /// Getter of the `container` attribute.
    pub fn container(&self) -> ScrollIntoViewContainer {
        self.inner.get("container").as_::<ScrollIntoViewContainer>()
    }

    /// Setter of the `container` attribute.
    pub fn set_container(&mut self, value: &ScrollIntoViewContainer) {
        self.inner.set("container", value);
    }
}
