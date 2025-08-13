use super::*;




/// The FocusOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FocusOptions {
    inner: Any,
}

impl FromVal for FocusOptions {
    fn from_val(v: &Any) -> Self {
        FocusOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FocusOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FocusOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FocusOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FocusOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FocusOptions> for Any {
    fn from(s: FocusOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FocusOptions> for Any {
    fn from(s: &FocusOptions) -> Any {
        s.inner.clone()
    }
}

impl FocusOptions {
    /// Getter of the `preventScroll` attribute.
    pub fn prevent_scroll(&self) -> bool {
        self.inner.get("preventScroll").as_::<bool>()
    }

    /// Setter of the `preventScroll` attribute.
    pub fn set_prevent_scroll(&mut self, value: bool) {
        self.inner.set("preventScroll", value);
    }
}
impl FocusOptions {
    /// Getter of the `focusVisible` attribute.
    pub fn focus_visible(&self) -> bool {
        self.inner.get("focusVisible").as_::<bool>()
    }

    /// Setter of the `focusVisible` attribute.
    pub fn set_focus_visible(&mut self, value: bool) {
        self.inner.set("focusVisible", value);
    }
}
