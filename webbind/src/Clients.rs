use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ClientQueryOptions {
    inner: emlite::Val,
}
impl FromVal for ClientQueryOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ClientQueryOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ClientQueryOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ClientQueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ClientQueryOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ClientQueryOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ClientQueryOptions> for emlite::Val {
    fn from(s: ClientQueryOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ClientQueryOptions> for emlite::Val {
    fn from(s: &ClientQueryOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl ClientQueryOptions {
    pub fn include_uncontrolled(&self) -> bool {
        self.inner.get("includeUncontrolled").as_::<bool>()
    }

    pub fn set_include_uncontrolled(&mut self, value: bool) {
        self.inner.set("includeUncontrolled", value);
    }
}
impl ClientQueryOptions {
    pub fn type_(&self) -> ClientType {
        self.inner.get("type").as_::<ClientType>()
    }

    pub fn set_type_(&mut self, value: ClientType) {
        self.inner.set("type", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Clients {
    inner: emlite::Val,
}
impl FromVal for Clients {
    fn from_val(v: &emlite::Val) -> Self {
        Clients {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Clients {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Clients {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Clients {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Clients {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Clients> for emlite::Val {
    fn from(s: Clients) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Clients> for emlite::Val {
    fn from(s: &Clients) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Clients);

impl Clients {
    pub fn get(&self, id: DOMString) -> Promise {
        self.inner.call("get", &[id.into()]).as_::<Promise>()
    }
}
impl Clients {
    pub fn match_all0(&self) -> Promise {
        self.inner.call("matchAll", &[]).as_::<Promise>()
    }

    pub fn match_all1(&self, options: ClientQueryOptions) -> Promise {
        self.inner
            .call("matchAll", &[options.into()])
            .as_::<Promise>()
    }
}
impl Clients {
    pub fn open_window(&self, url: USVString) -> Promise {
        self.inner
            .call("openWindow", &[url.into()])
            .as_::<Promise>()
    }
}
impl Clients {
    pub fn claim(&self) -> Promise {
        self.inner.call("claim", &[]).as_::<Promise>()
    }
}
