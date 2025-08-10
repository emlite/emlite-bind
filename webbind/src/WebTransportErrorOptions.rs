use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportErrorOptions {
    inner: Any,
}
impl FromVal for WebTransportErrorOptions {
    fn from_val(v: &Any) -> Self {
        WebTransportErrorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportErrorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportErrorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebTransportErrorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebTransportErrorOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebTransportErrorOptions> for Any {
    fn from(s: WebTransportErrorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebTransportErrorOptions> for Any {
    fn from(s: &WebTransportErrorOptions) -> Any {
        s.inner.clone()
    }
}

impl WebTransportErrorOptions {
    pub fn source(&self) -> WebTransportErrorSource {
        self.inner.get("source").as_::<WebTransportErrorSource>()
    }

    pub fn set_source(&mut self, value: &WebTransportErrorSource) {
        self.inner.set("source", value);
    }
}
impl WebTransportErrorOptions {
    pub fn stream_error_code(&self) -> u32 {
        self.inner.get("streamErrorCode").as_::<u32>()
    }

    pub fn set_stream_error_code(&mut self, value: u32) {
        self.inner.set("streamErrorCode", value);
    }
}
