use super::*;

/// The Client class.
/// [`Client`](https://developer.mozilla.org/en-US/docs/Web/API/Client)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Client {
    inner: Any,
}

impl FromVal for Client {
    fn from_val(v: &Any) -> Self {
        Client {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Client {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Client {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Client {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Client> for Any {
    fn from(s: Client) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Client> for Any {
    fn from(s: &Client) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Client);

impl Client {
    /// Getter of the `url` attribute.
    /// [`Client.url`](https://developer.mozilla.org/en-US/docs/Web/API/Client/url)
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }
}
impl Client {
    /// Getter of the `frameType` attribute.
    /// [`Client.frameType`](https://developer.mozilla.org/en-US/docs/Web/API/Client/frameType)
    pub fn frame_type(&self) -> FrameType {
        self.inner.get("frameType").as_::<FrameType>()
    }
}
impl Client {
    /// Getter of the `id` attribute.
    /// [`Client.id`](https://developer.mozilla.org/en-US/docs/Web/API/Client/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}
impl Client {
    /// Getter of the `type` attribute.
    /// [`Client.type`](https://developer.mozilla.org/en-US/docs/Web/API/Client/type)
    pub fn type_(&self) -> ClientType {
        self.inner.get("type").as_::<ClientType>()
    }
}
impl Client {
    /// The postMessage method.
    /// [`Client.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/Client/postMessage)
    pub fn post_message0(&self, message: &Any) -> Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<Undefined>()
    }
    /// The postMessage method.
    /// [`Client.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/Client/postMessage)
    pub fn post_message1(&self, message: &Any, options: &StructuredSerializeOptions) -> Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl Client {
    /// Getter of the `lifecycleState` attribute.
    /// [`Client.lifecycleState`](https://developer.mozilla.org/en-US/docs/Web/API/Client/lifecycleState)
    pub fn lifecycle_state(&self) -> ClientLifecycleState {
        self.inner
            .get("lifecycleState")
            .as_::<ClientLifecycleState>()
    }
}
