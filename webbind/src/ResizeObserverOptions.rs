use super::*;

/// The ResizeObserverOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ResizeObserverOptions {
    inner: Any,
}

impl FromVal for ResizeObserverOptions {
    fn from_val(v: &Any) -> Self {
        ResizeObserverOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ResizeObserverOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ResizeObserverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ResizeObserverOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ResizeObserverOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ResizeObserverOptions> for Any {
    fn from(s: ResizeObserverOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ResizeObserverOptions> for Any {
    fn from(s: &ResizeObserverOptions) -> Any {
        s.inner.clone()
    }
}

impl ResizeObserverOptions {
    /// Getter of the `box` attribute.
    pub fn box_(&self) -> ResizeObserverBoxOptions {
        self.inner.get("box").as_::<ResizeObserverBoxOptions>()
    }

    /// Setter of the `box` attribute.
    pub fn set_box_(&mut self, value: &ResizeObserverBoxOptions) {
        self.inner.set("box", value);
    }
}
