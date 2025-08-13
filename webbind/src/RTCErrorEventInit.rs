use super::*;




/// The RTCErrorEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCErrorEventInit {
    inner: Any,
}

impl FromVal for RTCErrorEventInit {
    fn from_val(v: &Any) -> Self {
        RTCErrorEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCErrorEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCErrorEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCErrorEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCErrorEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCErrorEventInit> for Any {
    fn from(s: RTCErrorEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCErrorEventInit> for Any {
    fn from(s: &RTCErrorEventInit) -> Any {
        s.inner.clone()
    }
}

impl RTCErrorEventInit {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> RTCError {
        self.inner.get("error").as_::<RTCError>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &RTCError) {
        self.inner.set("error", value);
    }
}
