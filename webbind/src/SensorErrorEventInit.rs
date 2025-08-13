use super::*;




/// The SensorErrorEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SensorErrorEventInit {
    inner: Any,
}

impl FromVal for SensorErrorEventInit {
    fn from_val(v: &Any) -> Self {
        SensorErrorEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SensorErrorEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SensorErrorEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SensorErrorEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SensorErrorEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SensorErrorEventInit> for Any {
    fn from(s: SensorErrorEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SensorErrorEventInit> for Any {
    fn from(s: &SensorErrorEventInit) -> Any {
        s.inner.clone()
    }
}

impl SensorErrorEventInit {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> DOMException {
        self.inner.get("error").as_::<DOMException>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &DOMException) {
        self.inner.set("error", value);
    }
}
