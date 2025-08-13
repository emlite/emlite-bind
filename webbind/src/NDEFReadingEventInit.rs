use super::*;




/// The NDEFReadingEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFReadingEventInit {
    inner: Any,
}

impl FromVal for NDEFReadingEventInit {
    fn from_val(v: &Any) -> Self {
        NDEFReadingEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NDEFReadingEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NDEFReadingEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NDEFReadingEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NDEFReadingEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<NDEFReadingEventInit> for Any {
    fn from(s: NDEFReadingEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NDEFReadingEventInit> for Any {
    fn from(s: &NDEFReadingEventInit) -> Any {
        s.inner.clone()
    }
}

impl NDEFReadingEventInit {
    /// Getter of the `serialNumber` attribute.
    pub fn serial_number(&self) -> JsString {
        self.inner.get("serialNumber").as_::<JsString>()
    }

    /// Setter of the `serialNumber` attribute.
    pub fn set_serial_number(&mut self, value: &JsString) {
        self.inner.set("serialNumber", value);
    }
}
impl NDEFReadingEventInit {
    /// Getter of the `message` attribute.
    pub fn message(&self) -> NDEFMessageInit {
        self.inner.get("message").as_::<NDEFMessageInit>()
    }

    /// Setter of the `message` attribute.
    pub fn set_message(&mut self, value: &NDEFMessageInit) {
        self.inner.set("message", value);
    }
}
