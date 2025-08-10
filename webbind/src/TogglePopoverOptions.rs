use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TogglePopoverOptions {
    inner: Any,
}
impl FromVal for TogglePopoverOptions {
    fn from_val(v: &Any) -> Self {
        TogglePopoverOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TogglePopoverOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TogglePopoverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TogglePopoverOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TogglePopoverOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TogglePopoverOptions> for Any {
    fn from(s: TogglePopoverOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TogglePopoverOptions> for Any {
    fn from(s: &TogglePopoverOptions) -> Any {
        s.inner.clone()
    }
}

impl TogglePopoverOptions {
    pub fn force(&self) -> bool {
        self.inner.get("force").as_::<bool>()
    }

    pub fn set_force(&mut self, value: bool) {
        self.inner.set("force", value);
    }
}
