use super::*;

/// The FetchEvent class.
/// [`FetchEvent`](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FetchEvent {
    inner: ExtendableEvent,
}
impl FromVal for FetchEvent {
    fn from_val(v: &Any) -> Self {
        FetchEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FetchEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FetchEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FetchEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FetchEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FetchEvent> for Any {
    fn from(s: FetchEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FetchEvent> for Any {
    fn from(s: &FetchEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FetchEvent);

impl FetchEvent {
    /// The `new FetchEvent(..)` constructor, creating a new FetchEvent instance
    pub fn new(type_: &str, event_init_dict: &Any) -> FetchEvent {
        Self {
            inner: Any::global("FetchEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl FetchEvent {
    /// Getter of the `request` attribute.
    /// [`FetchEvent.request`](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/request)
    pub fn request(&self) -> Request {
        self.inner.get("request").as_::<Request>()
    }
}
impl FetchEvent {
    /// Getter of the `preloadResponse` attribute.
    /// [`FetchEvent.preloadResponse`](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/preloadResponse)
    pub fn preload_response(&self) -> Promise {
        self.inner.get("preloadResponse").as_::<Promise>()
    }
}
impl FetchEvent {
    /// Getter of the `clientId` attribute.
    /// [`FetchEvent.clientId`](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/clientId)
    pub fn client_id(&self) -> String {
        self.inner.get("clientId").as_::<String>()
    }
}
impl FetchEvent {
    /// Getter of the `resultingClientId` attribute.
    /// [`FetchEvent.resultingClientId`](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/resultingClientId)
    pub fn resulting_client_id(&self) -> String {
        self.inner.get("resultingClientId").as_::<String>()
    }
}
impl FetchEvent {
    /// Getter of the `replacesClientId` attribute.
    /// [`FetchEvent.replacesClientId`](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/replacesClientId)
    pub fn replaces_client_id(&self) -> String {
        self.inner.get("replacesClientId").as_::<String>()
    }
}
impl FetchEvent {
    /// Getter of the `handled` attribute.
    /// [`FetchEvent.handled`](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/handled)
    pub fn handled(&self) -> Promise {
        self.inner.get("handled").as_::<Promise>()
    }
}
impl FetchEvent {
    /// The respondWith method.
    /// [`FetchEvent.respondWith`](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/respondWith)
    pub fn respond_with(&self, r: &Promise) -> Undefined {
        self.inner
            .call("respondWith", &[r.into()])
            .as_::<Undefined>()
    }
}
