use super::*;




/// The FetchEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FetchEventInit {
    inner: Any,
}

impl FromVal for FetchEventInit {
    fn from_val(v: &Any) -> Self {
        FetchEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FetchEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FetchEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FetchEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FetchEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FetchEventInit> for Any {
    fn from(s: FetchEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FetchEventInit> for Any {
    fn from(s: &FetchEventInit) -> Any {
        s.inner.clone()
    }
}

impl FetchEventInit {
    /// Getter of the `request` attribute.
    pub fn request(&self) -> Request {
        self.inner.get("request").as_::<Request>()
    }

    /// Setter of the `request` attribute.
    pub fn set_request(&mut self, value: &Request) {
        self.inner.set("request", value);
    }
}
impl FetchEventInit {
    /// Getter of the `preloadResponse` attribute.
    pub fn preload_response(&self) -> Promise<Any> {
        self.inner.get("preloadResponse").as_::<Promise<Any>>()
    }

    /// Setter of the `preloadResponse` attribute.
    pub fn set_preload_response(&mut self, value: &Promise<Any>) {
        self.inner.set("preloadResponse", value);
    }
}
impl FetchEventInit {
    /// Getter of the `clientId` attribute.
    pub fn client_id(&self) -> JsString {
        self.inner.get("clientId").as_::<JsString>()
    }

    /// Setter of the `clientId` attribute.
    pub fn set_client_id(&mut self, value: &JsString) {
        self.inner.set("clientId", value);
    }
}
impl FetchEventInit {
    /// Getter of the `resultingClientId` attribute.
    pub fn resulting_client_id(&self) -> JsString {
        self.inner.get("resultingClientId").as_::<JsString>()
    }

    /// Setter of the `resultingClientId` attribute.
    pub fn set_resulting_client_id(&mut self, value: &JsString) {
        self.inner.set("resultingClientId", value);
    }
}
impl FetchEventInit {
    /// Getter of the `replacesClientId` attribute.
    pub fn replaces_client_id(&self) -> JsString {
        self.inner.get("replacesClientId").as_::<JsString>()
    }

    /// Setter of the `replacesClientId` attribute.
    pub fn set_replaces_client_id(&mut self, value: &JsString) {
        self.inner.set("replacesClientId", value);
    }
}
impl FetchEventInit {
    /// Getter of the `handled` attribute.
    pub fn handled(&self) -> Promise<Undefined> {
        self.inner.get("handled").as_::<Promise<Undefined>>()
    }

    /// Setter of the `handled` attribute.
    pub fn set_handled(&mut self, value: &Promise<Undefined>) {
        self.inner.set("handled", value);
    }
}
