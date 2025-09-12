use super::*;

/// The Clients class.
/// [`Clients`](https://developer.mozilla.org/en-US/docs/Web/API/Clients)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Clients {
    inner: Any,
}

impl FromVal for Clients {
    fn from_val(v: &Any) -> Self {
        Clients {
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

impl core::ops::Deref for Clients {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Clients {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Clients {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Clients {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Clients> for Any {
    fn from(s: Clients) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Clients> for Any {
    fn from(s: &Clients) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Clients);

impl Clients {
    /// The get method.
    /// [`Clients.get`](https://developer.mozilla.org/en-US/docs/Web/API/Clients/get)
    pub fn get(&self, id: &JsString) -> Promise<Any> {
        self.inner.call("get", &[id.into()]).as_::<Promise<Any>>()
    }
}
impl Clients {
    /// The matchAll method.
    /// [`Clients.matchAll`](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)
    pub fn match_all(&self) -> Promise<TypedArray<Client>> {
        self.inner
            .call("matchAll", &[])
            .as_::<Promise<TypedArray<Client>>>()
    }
}
impl Clients {
    /// The matchAll method.
    /// [`Clients.matchAll`](https://developer.mozilla.org/en-US/docs/Web/API/Clients/matchAll)
    pub fn match_all_with_options(
        &self,
        options: &ClientQueryOptions,
    ) -> Promise<TypedArray<Client>> {
        self.inner
            .call("matchAll", &[options.into()])
            .as_::<Promise<TypedArray<Client>>>()
    }
}
impl Clients {
    /// The openWindow method.
    /// [`Clients.openWindow`](https://developer.mozilla.org/en-US/docs/Web/API/Clients/openWindow)
    pub fn open_window(&self, url: &JsString) -> Promise<WindowClient> {
        self.inner
            .call("openWindow", &[url.into()])
            .as_::<Promise<WindowClient>>()
    }
}
impl Clients {
    /// The claim method.
    /// [`Clients.claim`](https://developer.mozilla.org/en-US/docs/Web/API/Clients/claim)
    pub fn claim(&self) -> Promise<Undefined> {
        self.inner.call("claim", &[]).as_::<Promise<Undefined>>()
    }
}
