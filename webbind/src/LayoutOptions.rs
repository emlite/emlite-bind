use super::*;




/// The LayoutOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LayoutOptions {
    inner: Any,
}

impl FromVal for LayoutOptions {
    fn from_val(v: &Any) -> Self {
        LayoutOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for LayoutOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LayoutOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LayoutOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LayoutOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<LayoutOptions> for Any {
    fn from(s: LayoutOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LayoutOptions> for Any {
    fn from(s: &LayoutOptions) -> Any {
        s.inner.clone()
    }
}

impl LayoutOptions {
    /// Getter of the `childDisplay` attribute.
    pub fn child_display(&self) -> ChildDisplayType {
        self.inner.get("childDisplay").as_::<ChildDisplayType>()
    }

    /// Setter of the `childDisplay` attribute.
    pub fn set_child_display(&mut self, value: &ChildDisplayType) {
        self.inner.set("childDisplay", value);
    }
}
impl LayoutOptions {
    /// Getter of the `sizing` attribute.
    pub fn sizing(&self) -> LayoutSizingMode {
        self.inner.get("sizing").as_::<LayoutSizingMode>()
    }

    /// Setter of the `sizing` attribute.
    pub fn set_sizing(&mut self, value: &LayoutSizingMode) {
        self.inner.set("sizing", value);
    }
}
