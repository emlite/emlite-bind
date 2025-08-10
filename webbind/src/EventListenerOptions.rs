use super::*;

/// The EventListenerOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EventListenerOptions {
    inner: Any,
}

impl FromVal for EventListenerOptions {
    fn from_val(v: &Any) -> Self {
        EventListenerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EventListenerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EventListenerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EventListenerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EventListenerOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EventListenerOptions> for Any {
    fn from(s: EventListenerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EventListenerOptions> for Any {
    fn from(s: &EventListenerOptions) -> Any {
        s.inner.clone()
    }
}

impl EventListenerOptions {
    /// Getter of the `capture` attribute.
    pub fn capture(&self) -> bool {
        self.inner.get("capture").as_::<bool>()
    }

    /// Setter of the `capture` attribute.
    pub fn set_capture(&mut self, value: bool) {
        self.inner.set("capture", value);
    }
}
