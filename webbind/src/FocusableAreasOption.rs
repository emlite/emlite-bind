use super::*;

/// The FocusableAreasOption dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FocusableAreasOption {
    inner: Any,
}

impl FromVal for FocusableAreasOption {
    fn from_val(v: &Any) -> Self {
        FocusableAreasOption { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FocusableAreasOption {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FocusableAreasOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FocusableAreasOption {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FocusableAreasOption {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FocusableAreasOption> for Any {
    fn from(s: FocusableAreasOption) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FocusableAreasOption> for Any {
    fn from(s: &FocusableAreasOption) -> Any {
        s.inner.clone()
    }
}

impl FocusableAreasOption {
    /// Getter of the `mode` attribute.
    pub fn mode(&self) -> FocusableAreaSearchMode {
        self.inner.get("mode").as_::<FocusableAreaSearchMode>()
    }

    /// Setter of the `mode` attribute.
    pub fn set_mode(&mut self, value: &FocusableAreaSearchMode) {
        self.inner.set("mode", value);
    }
}
