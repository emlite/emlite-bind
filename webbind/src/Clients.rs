use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<ClientQueryOptions> for emlite::Val {
    fn from(s: ClientQueryOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
impl From<Clients> for emlite::Val {
    fn from(s: Clients) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Clients {
    pub fn get(&self, id: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("get", &[id.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Clients {
    pub fn match_all0(&self) -> jsbind::Promise {
        self.inner.call("matchAll", &[]).as_::<jsbind::Promise>()
    }

    pub fn match_all1(&self, options: ClientQueryOptions) -> jsbind::Promise {
        self.inner
            .call("matchAll", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Clients {
    pub fn open_window(&self, url: jsbind::USVString) -> jsbind::Promise {
        self.inner
            .call("openWindow", &[url.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Clients {
    pub fn claim(&self) -> jsbind::Promise {
        self.inner.call("claim", &[]).as_::<jsbind::Promise>()
    }
}
