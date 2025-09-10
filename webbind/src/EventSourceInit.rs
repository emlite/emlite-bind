use super::*;

/// The EventSourceInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EventSourceInit {
    inner: Any,
}

impl FromVal for EventSourceInit {
    fn from_val(v: &Any) -> Self {
        EventSourceInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EventSourceInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EventSourceInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EventSourceInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EventSourceInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EventSourceInit> for Any {
    fn from(s: EventSourceInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EventSourceInit> for Any {
    fn from(s: &EventSourceInit) -> Any {
        s.inner.clone()
    }
}

impl EventSourceInit {
    /// Getter of the `withCredentials` attribute.
    pub fn with_credentials(&self) -> bool {
        self.inner.get("withCredentials").as_::<bool>()
    }

    /// Setter of the `withCredentials` attribute.
    pub fn set_with_credentials(&mut self, value: bool) {
        self.inner.set("withCredentials", value);
    }
}
