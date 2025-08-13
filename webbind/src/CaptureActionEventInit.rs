use super::*;




/// The CaptureActionEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaptureActionEventInit {
    inner: Any,
}

impl FromVal for CaptureActionEventInit {
    fn from_val(v: &Any) -> Self {
        CaptureActionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CaptureActionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CaptureActionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CaptureActionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CaptureActionEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CaptureActionEventInit> for Any {
    fn from(s: CaptureActionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CaptureActionEventInit> for Any {
    fn from(s: &CaptureActionEventInit) -> Any {
        s.inner.clone()
    }
}

impl CaptureActionEventInit {
    /// Getter of the `action` attribute.
    pub fn action(&self) -> JsString {
        self.inner.get("action").as_::<JsString>()
    }

    /// Setter of the `action` attribute.
    pub fn set_action(&mut self, value: &JsString) {
        self.inner.set("action", value);
    }
}
