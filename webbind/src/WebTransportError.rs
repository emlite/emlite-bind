use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WebTransportError {
    inner: DOMException,
}
impl FromVal for WebTransportError {
    fn from_val(v: &emlite::Val) -> Self {
        WebTransportError {
            inner: DOMException::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebTransportError {
    type Target = DOMException;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebTransportError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebTransportError> for emlite::Val {
    fn from(s: WebTransportError) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebTransportError {
    pub fn new0() -> WebTransportError {
        Self {
            inner: emlite::Val::global("WebTransportError")
                .new(&[])
                .as_::<DOMException>(),
        }
    }

    pub fn new1(message: jsbind::DOMString) -> WebTransportError {
        Self {
            inner: emlite::Val::global("WebTransportError")
                .new(&[message.into()])
                .as_::<DOMException>(),
        }
    }

    pub fn new2(message: jsbind::DOMString, options: jsbind::Any) -> WebTransportError {
        Self {
            inner: emlite::Val::global("WebTransportError")
                .new(&[message.into(), options.into()])
                .as_::<DOMException>(),
        }
    }
}
impl WebTransportError {
    pub fn source(&self) -> WebTransportErrorSource {
        self.inner.get("source").as_::<WebTransportErrorSource>()
    }
}
impl WebTransportError {
    pub fn stream_error_code(&self) -> u32 {
        self.inner.get("streamErrorCode").as_::<u32>()
    }
}
