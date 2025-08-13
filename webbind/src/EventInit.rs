use super::*;




/// The EventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EventInit {
    inner: Any,
}

impl FromVal for EventInit {
    fn from_val(v: &Any) -> Self {
        EventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for EventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<EventInit> for Any {
    fn from(s: EventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EventInit> for Any {
    fn from(s: &EventInit) -> Any {
        s.inner.clone()
    }
}

impl EventInit {
    /// Getter of the `bubbles` attribute.
    pub fn bubbles(&self) -> bool {
        self.inner.get("bubbles").as_::<bool>()
    }

    /// Setter of the `bubbles` attribute.
    pub fn set_bubbles(&mut self, value: bool) {
        self.inner.set("bubbles", value);
    }
}
impl EventInit {
    /// Getter of the `cancelable` attribute.
    pub fn cancelable(&self) -> bool {
        self.inner.get("cancelable").as_::<bool>()
    }

    /// Setter of the `cancelable` attribute.
    pub fn set_cancelable(&mut self, value: bool) {
        self.inner.set("cancelable", value);
    }
}
impl EventInit {
    /// Getter of the `composed` attribute.
    pub fn composed(&self) -> bool {
        self.inner.get("composed").as_::<bool>()
    }

    /// Setter of the `composed` attribute.
    pub fn set_composed(&mut self, value: bool) {
        self.inner.set("composed", value);
    }
}
