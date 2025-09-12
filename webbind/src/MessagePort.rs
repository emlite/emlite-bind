use super::*;

/// The MessagePort class.
/// [`MessagePort`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MessagePort {
    inner: EventTarget,
}

impl FromVal for MessagePort {
    fn from_val(v: &Any) -> Self {
        MessagePort {
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

impl core::ops::Deref for MessagePort {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MessagePort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MessagePort {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MessagePort {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MessagePort> for Any {
    fn from(s: MessagePort) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MessagePort> for Any {
    fn from(s: &MessagePort) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MessagePort);

impl MessagePort {
    /// Getter of the `onclose` attribute.
    /// [`MessagePort.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onclose)
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    /// Setter of the `onclose` attribute.
    /// [`MessagePort.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onclose)
    pub fn set_onclose(&mut self, value: &Any) {
        self.inner.set("onclose", value);
    }
}
impl MessagePort {
    /// Getter of the `onmessage` attribute.
    /// [`MessagePort.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`MessagePort.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl MessagePort {
    /// Getter of the `onmessageerror` attribute.
    /// [`MessagePort.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessageerror)
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    /// Setter of the `onmessageerror` attribute.
    /// [`MessagePort.onmessageerror`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessageerror)
    pub fn set_onmessageerror(&mut self, value: &Any) {
        self.inner.set("onmessageerror", value);
    }
}
impl MessagePort {
    /// The postMessage method.
    /// [`MessagePort.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/postMessage)
    pub fn post_message(&self, message: &Any, transfer: &TypedArray<Object>) -> Undefined {
        self.inner
            .call("postMessage", &[message.into(), transfer.into()])
            .as_::<Undefined>()
    }
}
impl MessagePort {
    /// The postMessage method.
    /// [`MessagePort.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/postMessage)
    pub fn post_message1(&self, message: &Any) -> Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<Undefined>()
    }
    /// The postMessage method.
    /// [`MessagePort.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/postMessage)
    pub fn post_message2(&self, message: &Any, options: &StructuredSerializeOptions) -> Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl MessagePort {
    /// The start method.
    /// [`MessagePort.start`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/start)
    pub fn start(&self) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }
}
impl MessagePort {
    /// The close method.
    /// [`MessagePort.close`](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
