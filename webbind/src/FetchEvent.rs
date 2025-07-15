use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FetchEvent {
    inner: ExtendableEvent,
}
impl FromVal for FetchEvent {
    fn from_val(v: &emlite::Val) -> Self {
        FetchEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for FetchEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FetchEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FetchEvent> for emlite::Val {
    fn from(s: FetchEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FetchEvent);

impl FetchEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> FetchEvent {
        Self {
            inner: emlite::Val::global("FetchEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl FetchEvent {
    pub fn request(&self) -> Request {
        self.inner.get("request").as_::<Request>()
    }
}
impl FetchEvent {
    pub fn preload_response(&self) -> Promise {
        self.inner.get("preloadResponse").as_::<Promise>()
    }
}
impl FetchEvent {
    pub fn client_id(&self) -> DOMString {
        self.inner.get("clientId").as_::<DOMString>()
    }
}
impl FetchEvent {
    pub fn resulting_client_id(&self) -> DOMString {
        self.inner.get("resultingClientId").as_::<DOMString>()
    }
}
impl FetchEvent {
    pub fn replaces_client_id(&self) -> DOMString {
        self.inner.get("replacesClientId").as_::<DOMString>()
    }
}
impl FetchEvent {
    pub fn handled(&self) -> Promise {
        self.inner.get("handled").as_::<Promise>()
    }
}
impl FetchEvent {
    pub fn respond_with(&self, r: Promise) -> Undefined {
        self.inner
            .call("respondWith", &[r.into()])
            .as_::<Undefined>()
    }
}
