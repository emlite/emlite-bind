use super::*;

/// The MessageEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MessageEventInit {
    inner: Any,
}

impl FromVal for MessageEventInit {
    fn from_val(v: &Any) -> Self {
        MessageEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MessageEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MessageEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MessageEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MessageEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MessageEventInit> for Any {
    fn from(s: MessageEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MessageEventInit> for Any {
    fn from(s: &MessageEventInit) -> Any {
        s.inner.clone()
    }
}

impl MessageEventInit {
    /// Getter of the `data` attribute.
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    /// Setter of the `data` attribute.
    pub fn set_data(&mut self, value: &Any) {
        self.inner.set("data", value);
    }
}
impl MessageEventInit {
    /// Getter of the `origin` attribute.
    pub fn origin(&self) -> JsString {
        self.inner.get("origin").as_::<JsString>()
    }

    /// Setter of the `origin` attribute.
    pub fn set_origin(&mut self, value: &JsString) {
        self.inner.set("origin", value);
    }
}
impl MessageEventInit {
    /// Getter of the `lastEventId` attribute.
    pub fn last_event_id(&self) -> JsString {
        self.inner.get("lastEventId").as_::<JsString>()
    }

    /// Setter of the `lastEventId` attribute.
    pub fn set_last_event_id(&mut self, value: &JsString) {
        self.inner.set("lastEventId", value);
    }
}
impl MessageEventInit {
    /// Getter of the `source` attribute.
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }

    /// Setter of the `source` attribute.
    pub fn set_source(&mut self, value: &Any) {
        self.inner.set("source", value);
    }
}
impl MessageEventInit {
    /// Getter of the `ports` attribute.
    pub fn ports(&self) -> TypedArray<MessagePort> {
        self.inner.get("ports").as_::<TypedArray<MessagePort>>()
    }

    /// Setter of the `ports` attribute.
    pub fn set_ports(&mut self, value: &TypedArray<MessagePort>) {
        self.inner.set("ports", value);
    }
}
