use super::*;

/// The WebTransportError class.
/// [`WebTransportError`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebTransportError {
    inner: DOMException,
}
impl FromVal for WebTransportError {
    fn from_val(v: &Any) -> Self {
        WebTransportError {
            inner: DOMException::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for WebTransportError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebTransportError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebTransportError> for Any {
    fn from(s: WebTransportError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebTransportError> for Any {
    fn from(s: &WebTransportError) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebTransportError);

impl WebTransportError {
    /// The `new WebTransportError(..)` constructor, creating a new WebTransportError instance
    pub fn new0() -> WebTransportError {
        Self {
            inner: Any::global("WebTransportError")
                .new(&[])
                .as_::<DOMException>(),
        }
    }

    /// The `new WebTransportError(..)` constructor, creating a new WebTransportError instance
    pub fn new1(message: &JsString) -> WebTransportError {
        Self {
            inner: Any::global("WebTransportError")
                .new(&[message.into()])
                .as_::<DOMException>(),
        }
    }

    /// The `new WebTransportError(..)` constructor, creating a new WebTransportError instance
    pub fn new2(message: &JsString, options: &WebTransportErrorOptions) -> WebTransportError {
        Self {
            inner: Any::global("WebTransportError")
                .new(&[message.into(), options.into()])
                .as_::<DOMException>(),
        }
    }
}
impl WebTransportError {
    /// Getter of the `source` attribute.
    /// [`WebTransportError.source`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/source)
    pub fn source(&self) -> WebTransportErrorSource {
        self.inner.get("source").as_::<WebTransportErrorSource>()
    }
}
impl WebTransportError {
    /// Getter of the `streamErrorCode` attribute.
    /// [`WebTransportError.streamErrorCode`](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/streamErrorCode)
    pub fn stream_error_code(&self) -> u32 {
        self.inner.get("streamErrorCode").as_::<u32>()
    }
}
