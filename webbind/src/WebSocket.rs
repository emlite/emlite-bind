use super::*;

/// The WebSocket class.
/// [`WebSocket`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebSocket {
    inner: EventTarget,
}

impl FromVal for WebSocket {
    fn from_val(v: &Any) -> Self {
        WebSocket {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebSocket {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebSocket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebSocket {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebSocket {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebSocket> for Any {
    fn from(s: WebSocket) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebSocket> for Any {
    fn from(s: &WebSocket) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebSocket);

impl WebSocket {
    /// Getter of the `url` attribute.
    /// [`WebSocket.url`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}
impl WebSocket {
    /// Getter of the `readyState` attribute.
    /// [`WebSocket.readyState`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState)
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl WebSocket {
    /// Getter of the `bufferedAmount` attribute.
    /// [`WebSocket.bufferedAmount`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/bufferedAmount)
    pub fn buffered_amount(&self) -> u64 {
        self.inner.get("bufferedAmount").as_::<u64>()
    }
}
impl WebSocket {
    /// Getter of the `onopen` attribute.
    /// [`WebSocket.onopen`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)
    pub fn onopen(&self) -> Any {
        self.inner.get("onopen").as_::<Any>()
    }

    /// Setter of the `onopen` attribute.
    /// [`WebSocket.onopen`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)
    pub fn set_onopen(&mut self, value: &Any) {
        self.inner.set("onopen", value);
    }
}
impl WebSocket {
    /// Getter of the `onerror` attribute.
    /// [`WebSocket.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`WebSocket.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl WebSocket {
    /// Getter of the `onclose` attribute.
    /// [`WebSocket.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    /// Setter of the `onclose` attribute.
    /// [`WebSocket.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)
    pub fn set_onclose(&mut self, value: &Any) {
        self.inner.set("onclose", value);
    }
}
impl WebSocket {
    /// Getter of the `extensions` attribute.
    /// [`WebSocket.extensions`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/extensions)
    pub fn extensions(&self) -> JsString {
        self.inner.get("extensions").as_::<JsString>()
    }
}
impl WebSocket {
    /// Getter of the `protocol` attribute.
    /// [`WebSocket.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/protocol)
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }
}
impl WebSocket {
    /// Getter of the `onmessage` attribute.
    /// [`WebSocket.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`WebSocket.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl WebSocket {
    /// Getter of the `binaryType` attribute.
    /// [`WebSocket.binaryType`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)
    pub fn binary_type(&self) -> BinaryType {
        self.inner.get("binaryType").as_::<BinaryType>()
    }

    /// Setter of the `binaryType` attribute.
    /// [`WebSocket.binaryType`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)
    pub fn set_binary_type(&mut self, value: &BinaryType) {
        self.inner.set("binaryType", value);
    }
}

impl WebSocket {
    /// The `new WebSocket(..)` constructor, creating a new WebSocket instance
    pub fn new(url: &JsString) -> WebSocket {
        Self {
            inner: Any::global("WebSocket")
                .new(&[url.into()])
                .as_::<EventTarget>(),
        }
    }
}

impl WebSocket {
    /// The `new WebSocket(..)` constructor, creating a new WebSocket instance
    pub fn new_with_protocols(url: &JsString, protocols: &Any) -> WebSocket {
        Self {
            inner: Any::global("WebSocket")
                .new(&[url.into(), protocols.into()])
                .as_::<EventTarget>(),
        }
    }
}

impl WebSocket {
    /// The close method.
    /// [`WebSocket.close`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl WebSocket {
    /// The close method.
    /// [`WebSocket.close`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)
    pub fn close_with_code(&self, code: u16) -> Undefined {
        self.inner.call("close", &[code.into()]).as_::<Undefined>()
    }
}
impl WebSocket {
    /// The close method.
    /// [`WebSocket.close`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)
    pub fn close_with_code_and_reason(&self, code: u16, reason: &JsString) -> Undefined {
        self.inner
            .call("close", &[code.into(), reason.into()])
            .as_::<Undefined>()
    }
}
impl WebSocket {
    /// The send method.
    /// [`WebSocket.send`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)
    pub fn send(&self, data: &Any) -> Undefined {
        self.inner.call("send", &[data.into()]).as_::<Undefined>()
    }
}
