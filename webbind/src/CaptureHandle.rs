use super::*;




/// The CaptureHandle dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaptureHandle {
    inner: Any,
}

impl FromVal for CaptureHandle {
    fn from_val(v: &Any) -> Self {
        CaptureHandle { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CaptureHandle {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CaptureHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CaptureHandle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CaptureHandle {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CaptureHandle> for Any {
    fn from(s: CaptureHandle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CaptureHandle> for Any {
    fn from(s: &CaptureHandle) -> Any {
        s.inner.clone()
    }
}

impl CaptureHandle {
    /// Getter of the `origin` attribute.
    pub fn origin(&self) -> JsString {
        self.inner.get("origin").as_::<JsString>()
    }

    /// Setter of the `origin` attribute.
    pub fn set_origin(&mut self, value: &JsString) {
        self.inner.set("origin", value);
    }
}
impl CaptureHandle {
    /// Getter of the `handle` attribute.
    pub fn handle(&self) -> JsString {
        self.inner.get("handle").as_::<JsString>()
    }

    /// Setter of the `handle` attribute.
    pub fn set_handle(&mut self, value: &JsString) {
        self.inner.set("handle", value);
    }
}
