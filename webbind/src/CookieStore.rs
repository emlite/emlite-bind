use super::*;

/// The CookieStore class.
/// [`CookieStore`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieStore {
    inner: EventTarget,
}

impl FromVal for CookieStore {
    fn from_val(v: &Any) -> Self {
        CookieStore {
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

impl core::ops::Deref for CookieStore {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CookieStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CookieStore {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CookieStore {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CookieStore> for Any {
    fn from(s: CookieStore) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CookieStore> for Any {
    fn from(s: &CookieStore) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CookieStore);

impl CookieStore {
    /// Getter of the `onchange` attribute.
    /// [`CookieStore.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`CookieStore.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
impl CookieStore {
    /// The get method.
    /// [`CookieStore.get`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get)
    pub fn get(&self, name: &JsString) -> Promise<CookieListItem> {
        self.inner
            .call("get", &[name.into()])
            .as_::<Promise<CookieListItem>>()
    }
}
impl CookieStore {
    /// The get method.
    /// [`CookieStore.get`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get)
    pub fn get1(&self) -> Promise<CookieListItem> {
        self.inner.call("get", &[]).as_::<Promise<CookieListItem>>()
    }
    /// The get method.
    /// [`CookieStore.get`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/get)
    pub fn get2(&self, options: &CookieStoreGetOptions) -> Promise<CookieListItem> {
        self.inner
            .call("get", &[options.into()])
            .as_::<Promise<CookieListItem>>()
    }
}
impl CookieStore {
    /// The getAll method.
    /// [`CookieStore.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll)
    pub fn get_all(&self, name: &JsString) -> Promise<Any> {
        self.inner
            .call("getAll", &[name.into()])
            .as_::<Promise<Any>>()
    }
}
impl CookieStore {
    /// The getAll method.
    /// [`CookieStore.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll)
    pub fn get_all1(&self) -> Promise<Any> {
        self.inner.call("getAll", &[]).as_::<Promise<Any>>()
    }
    /// The getAll method.
    /// [`CookieStore.getAll`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/getAll)
    pub fn get_all2(&self, options: &CookieStoreGetOptions) -> Promise<Any> {
        self.inner
            .call("getAll", &[options.into()])
            .as_::<Promise<Any>>()
    }
}
impl CookieStore {
    /// The set method.
    /// [`CookieStore.set`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/set)
    pub fn set(&self, name: &JsString, value: &JsString) -> Promise<Undefined> {
        self.inner
            .call("set", &[name.into(), value.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl CookieStore {
    /// The set method.
    /// [`CookieStore.set`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/set)
    pub fn set1(&self, options: &CookieInit) -> Promise<Undefined> {
        self.inner
            .call("set", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl CookieStore {
    /// The delete method.
    /// [`CookieStore.delete`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/delete)
    pub fn delete(&self, name: &JsString) -> Promise<Undefined> {
        self.inner
            .call("delete", &[name.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl CookieStore {
    /// The delete method.
    /// [`CookieStore.delete`](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore/delete)
    pub fn delete1(&self, options: &CookieStoreDeleteOptions) -> Promise<Undefined> {
        self.inner
            .call("delete", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
