use super::*;

/// The PresentationRequest class.
/// [`PresentationRequest`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PresentationRequest {
    inner: EventTarget,
}

impl FromVal for PresentationRequest {
    fn from_val(v: &Any) -> Self {
        PresentationRequest {
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

impl core::ops::Deref for PresentationRequest {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PresentationRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PresentationRequest {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PresentationRequest {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PresentationRequest> for Any {
    fn from(s: PresentationRequest) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PresentationRequest> for Any {
    fn from(s: &PresentationRequest) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PresentationRequest);

impl PresentationRequest {
    /// Getter of the `onconnectionavailable` attribute.
    /// [`PresentationRequest.onconnectionavailable`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)
    pub fn onconnectionavailable(&self) -> Any {
        self.inner.get("onconnectionavailable").as_::<Any>()
    }

    /// Setter of the `onconnectionavailable` attribute.
    /// [`PresentationRequest.onconnectionavailable`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)
    pub fn set_onconnectionavailable(&mut self, value: &Any) {
        self.inner.set("onconnectionavailable", value);
    }
}

impl PresentationRequest {
    /// The `new PresentationRequest(..)` constructor, creating a new PresentationRequest instance
    pub fn new(url: &JsString) -> PresentationRequest {
        Self {
            inner: Any::global("PresentationRequest")
                .new(&[url.into()])
                .as_::<EventTarget>(),
        }
    }
}

impl PresentationRequest {
    /// The `new PresentationRequest(..)` constructor, creating a new PresentationRequest instance
    pub fn new_with_urls(urls: &TypedArray<JsString>) -> PresentationRequest {
        Self {
            inner: Any::global("PresentationRequest")
                .new(&[urls.into()])
                .as_::<EventTarget>(),
        }
    }
}

impl PresentationRequest {
    /// The start method.
    /// [`PresentationRequest.start`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/start)
    pub fn start(&self) -> Promise<PresentationConnection> {
        self.inner
            .call("start", &[])
            .as_::<Promise<PresentationConnection>>()
    }
}
impl PresentationRequest {
    /// The reconnect method.
    /// [`PresentationRequest.reconnect`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/reconnect)
    pub fn reconnect(&self, presentation_id: &JsString) -> Promise<PresentationConnection> {
        self.inner
            .call("reconnect", &[presentation_id.into()])
            .as_::<Promise<PresentationConnection>>()
    }
}
impl PresentationRequest {
    /// The getAvailability method.
    /// [`PresentationRequest.getAvailability`](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/getAvailability)
    pub fn get_availability(&self) -> Promise<PresentationAvailability> {
        self.inner
            .call("getAvailability", &[])
            .as_::<Promise<PresentationAvailability>>()
    }
}
