use super::*;

/// The PresentationConnection class.
/// [`PresentationConnection`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationConnection {
    inner: EventTarget,
}
impl FromVal for PresentationConnection {
    fn from_val(v: &Any) -> Self {
        PresentationConnection {
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
impl core::ops::Deref for PresentationConnection {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PresentationConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PresentationConnection {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PresentationConnection {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PresentationConnection> for Any {
    fn from(s: PresentationConnection) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PresentationConnection> for Any {
    fn from(s: &PresentationConnection) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PresentationConnection);

impl PresentationConnection {
    /// Getter of the `id` attribute.
    /// [`PresentationConnection.id`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/id)
    pub fn id(&self) -> String {
        self.inner.get("id").as_::<String>()
    }
}
impl PresentationConnection {
    /// Getter of the `url` attribute.
    /// [`PresentationConnection.url`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/url)
    pub fn url(&self) -> String {
        self.inner.get("url").as_::<String>()
    }
}
impl PresentationConnection {
    /// Getter of the `state` attribute.
    /// [`PresentationConnection.state`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/state)
    pub fn state(&self) -> PresentationConnectionState {
        self.inner.get("state").as_::<PresentationConnectionState>()
    }
}
impl PresentationConnection {
    /// The close method.
    /// [`PresentationConnection.close`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl PresentationConnection {
    /// The terminate method.
    /// [`PresentationConnection.terminate`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/terminate)
    pub fn terminate(&self) -> Undefined {
        self.inner.call("terminate", &[]).as_::<Undefined>()
    }
}
impl PresentationConnection {
    /// Getter of the `onconnect` attribute.
    /// [`PresentationConnection.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    /// Setter of the `onconnect` attribute.
    /// [`PresentationConnection.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)
    pub fn set_onconnect(&mut self, value: &Any) {
        self.inner.set("onconnect", value);
    }
}
impl PresentationConnection {
    /// Getter of the `onclose` attribute.
    /// [`PresentationConnection.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    /// Setter of the `onclose` attribute.
    /// [`PresentationConnection.onclose`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)
    pub fn set_onclose(&mut self, value: &Any) {
        self.inner.set("onclose", value);
    }
}
impl PresentationConnection {
    /// Getter of the `onterminate` attribute.
    /// [`PresentationConnection.onterminate`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)
    pub fn onterminate(&self) -> Any {
        self.inner.get("onterminate").as_::<Any>()
    }

    /// Setter of the `onterminate` attribute.
    /// [`PresentationConnection.onterminate`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)
    pub fn set_onterminate(&mut self, value: &Any) {
        self.inner.set("onterminate", value);
    }
}
impl PresentationConnection {
    /// Getter of the `binaryType` attribute.
    /// [`PresentationConnection.binaryType`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)
    pub fn binary_type(&self) -> BinaryType {
        self.inner.get("binaryType").as_::<BinaryType>()
    }

    /// Setter of the `binaryType` attribute.
    /// [`PresentationConnection.binaryType`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)
    pub fn set_binary_type(&mut self, value: &BinaryType) {
        self.inner.set("binaryType", value);
    }
}
impl PresentationConnection {
    /// Getter of the `onmessage` attribute.
    /// [`PresentationConnection.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`PresentationConnection.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl PresentationConnection {
    /// The send method.
    /// [`PresentationConnection.send`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)
    pub fn send(&self, data: &Any) -> Undefined {
        self.inner.call("send", &[data.into()]).as_::<Undefined>()
    }
}
