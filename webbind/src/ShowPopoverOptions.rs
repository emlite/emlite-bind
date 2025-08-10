use super::*;

/// The ShowPopoverOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShowPopoverOptions {
    inner: Any,
}

impl FromVal for ShowPopoverOptions {
    fn from_val(v: &Any) -> Self {
        ShowPopoverOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ShowPopoverOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ShowPopoverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ShowPopoverOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ShowPopoverOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ShowPopoverOptions> for Any {
    fn from(s: ShowPopoverOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ShowPopoverOptions> for Any {
    fn from(s: &ShowPopoverOptions) -> Any {
        s.inner.clone()
    }
}

impl ShowPopoverOptions {
    /// Getter of the `source` attribute.
    pub fn source(&self) -> HTMLElement {
        self.inner.get("source").as_::<HTMLElement>()
    }

    /// Setter of the `source` attribute.
    pub fn set_source(&mut self, value: &HTMLElement) {
        self.inner.set("source", value);
    }
}
