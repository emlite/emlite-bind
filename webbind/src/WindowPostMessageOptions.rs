use super::*;




/// The WindowPostMessageOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WindowPostMessageOptions {
    inner: Any,
}

impl FromVal for WindowPostMessageOptions {
    fn from_val(v: &Any) -> Self {
        WindowPostMessageOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WindowPostMessageOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WindowPostMessageOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WindowPostMessageOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WindowPostMessageOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WindowPostMessageOptions> for Any {
    fn from(s: WindowPostMessageOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WindowPostMessageOptions> for Any {
    fn from(s: &WindowPostMessageOptions) -> Any {
        s.inner.clone()
    }
}

impl WindowPostMessageOptions {
    /// Getter of the `targetOrigin` attribute.
    pub fn target_origin(&self) -> JsString {
        self.inner.get("targetOrigin").as_::<JsString>()
    }

    /// Setter of the `targetOrigin` attribute.
    pub fn set_target_origin(&mut self, value: &JsString) {
        self.inner.set("targetOrigin", value);
    }
}
