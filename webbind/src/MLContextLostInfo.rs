use super::*;




/// The MLContextLostInfo dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLContextLostInfo {
    inner: Any,
}

impl FromVal for MLContextLostInfo {
    fn from_val(v: &Any) -> Self {
        MLContextLostInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLContextLostInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLContextLostInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLContextLostInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLContextLostInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLContextLostInfo> for Any {
    fn from(s: MLContextLostInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLContextLostInfo> for Any {
    fn from(s: &MLContextLostInfo) -> Any {
        s.inner.clone()
    }
}

impl MLContextLostInfo {
    /// Getter of the `message` attribute.
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }

    /// Setter of the `message` attribute.
    pub fn set_message(&mut self, value: &JsString) {
        self.inner.set("message", value);
    }
}
