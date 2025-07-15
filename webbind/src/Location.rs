use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Location {
    inner: emlite::Val,
}
impl FromVal for Location {
    fn from_val(v: &emlite::Val) -> Self {
        Location {
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
impl core::ops::Deref for Location {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Location {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Location {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Location {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Location> for emlite::Val {
    fn from(s: Location) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Location> for emlite::Val {
    fn from(s: &Location) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Location);

impl Location {
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }

    pub fn set_href(&mut self, value: USVString) {
        self.inner.set("href", value);
    }
}
impl Location {
    pub fn origin(&self) -> USVString {
        self.inner.get("origin").as_::<USVString>()
    }
}
impl Location {
    pub fn protocol(&self) -> USVString {
        self.inner.get("protocol").as_::<USVString>()
    }

    pub fn set_protocol(&mut self, value: USVString) {
        self.inner.set("protocol", value);
    }
}
impl Location {
    pub fn host(&self) -> USVString {
        self.inner.get("host").as_::<USVString>()
    }

    pub fn set_host(&mut self, value: USVString) {
        self.inner.set("host", value);
    }
}
impl Location {
    pub fn hostname(&self) -> USVString {
        self.inner.get("hostname").as_::<USVString>()
    }

    pub fn set_hostname(&mut self, value: USVString) {
        self.inner.set("hostname", value);
    }
}
impl Location {
    pub fn port(&self) -> USVString {
        self.inner.get("port").as_::<USVString>()
    }

    pub fn set_port(&mut self, value: USVString) {
        self.inner.set("port", value);
    }
}
impl Location {
    pub fn pathname(&self) -> USVString {
        self.inner.get("pathname").as_::<USVString>()
    }

    pub fn set_pathname(&mut self, value: USVString) {
        self.inner.set("pathname", value);
    }
}
impl Location {
    pub fn search(&self) -> USVString {
        self.inner.get("search").as_::<USVString>()
    }

    pub fn set_search(&mut self, value: USVString) {
        self.inner.set("search", value);
    }
}
impl Location {
    pub fn hash(&self) -> USVString {
        self.inner.get("hash").as_::<USVString>()
    }

    pub fn set_hash(&mut self, value: USVString) {
        self.inner.set("hash", value);
    }
}
impl Location {
    pub fn assign(&self, url: USVString) -> Undefined {
        self.inner.call("assign", &[url.into()]).as_::<Undefined>()
    }
}
impl Location {
    pub fn replace(&self, url: USVString) -> Undefined {
        self.inner.call("replace", &[url.into()]).as_::<Undefined>()
    }
}
impl Location {
    pub fn reload(&self) -> Undefined {
        self.inner.call("reload", &[]).as_::<Undefined>()
    }
}
impl Location {
    pub fn ancestor_origins(&self) -> DOMStringList {
        self.inner.get("ancestorOrigins").as_::<DOMStringList>()
    }
}
