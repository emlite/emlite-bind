use super::*;

/// The ObservableEventListenerOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ObservableEventListenerOptions {
    inner: Any,
}

impl FromVal for ObservableEventListenerOptions {
    fn from_val(v: &Any) -> Self {
        ObservableEventListenerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ObservableEventListenerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ObservableEventListenerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ObservableEventListenerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ObservableEventListenerOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ObservableEventListenerOptions> for Any {
    fn from(s: ObservableEventListenerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ObservableEventListenerOptions> for Any {
    fn from(s: &ObservableEventListenerOptions) -> Any {
        s.inner.clone()
    }
}

impl ObservableEventListenerOptions {
    /// Getter of the `capture` attribute.
    pub fn capture(&self) -> bool {
        self.inner.get("capture").as_::<bool>()
    }

    /// Setter of the `capture` attribute.
    pub fn set_capture(&mut self, value: bool) {
        self.inner.set("capture", value);
    }
}
impl ObservableEventListenerOptions {
    /// Getter of the `passive` attribute.
    pub fn passive(&self) -> bool {
        self.inner.get("passive").as_::<bool>()
    }

    /// Setter of the `passive` attribute.
    pub fn set_passive(&mut self, value: bool) {
        self.inner.set("passive", value);
    }
}
