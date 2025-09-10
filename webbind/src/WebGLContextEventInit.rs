use super::*;

/// The WebGLContextEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLContextEventInit {
    inner: Any,
}

impl FromVal for WebGLContextEventInit {
    fn from_val(v: &Any) -> Self {
        WebGLContextEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebGLContextEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLContextEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLContextEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLContextEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebGLContextEventInit> for Any {
    fn from(s: WebGLContextEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLContextEventInit> for Any {
    fn from(s: &WebGLContextEventInit) -> Any {
        s.inner.clone()
    }
}

impl WebGLContextEventInit {
    /// Getter of the `statusMessage` attribute.
    pub fn status_message(&self) -> JsString {
        self.inner.get("statusMessage").as_::<JsString>()
    }

    /// Setter of the `statusMessage` attribute.
    pub fn set_status_message(&mut self, value: &JsString) {
        self.inner.set("statusMessage", value);
    }
}
