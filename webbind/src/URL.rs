use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URL {
    inner: emlite::Val,
}
impl FromVal for URL {
    fn from_val(v: &emlite::Val) -> Self {
        URL {
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
impl core::ops::Deref for URL {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for URL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for URL {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for URL {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<URL> for emlite::Val {
    fn from(s: URL) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(URL);

impl URL {
    pub fn new0(url: USVString) -> URL {
        Self {
            inner: emlite::Val::global("URL")
                .new(&[url.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(url: USVString, base: USVString) -> URL {
        Self {
            inner: emlite::Val::global("URL")
                .new(&[url.into(), base.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl URL {
    pub fn parse0(url: USVString) -> URL {
        emlite::Val::global("URL")
            .call("parse", &[url.into()])
            .as_::<URL>()
    }

    pub fn parse1(url: USVString, base: USVString) -> URL {
        emlite::Val::global("URL")
            .call("parse", &[url.into(), base.into()])
            .as_::<URL>()
    }
}
impl URL {
    pub fn can_parse0(url: USVString) -> bool {
        emlite::Val::global("URL")
            .call("canParse", &[url.into()])
            .as_::<bool>()
    }

    pub fn can_parse1(url: USVString, base: USVString) -> bool {
        emlite::Val::global("URL")
            .call("canParse", &[url.into(), base.into()])
            .as_::<bool>()
    }
}
impl URL {
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }

    pub fn set_href(&mut self, value: USVString) {
        self.inner.set("href", value);
    }
}
impl URL {
    pub fn origin(&self) -> USVString {
        self.inner.get("origin").as_::<USVString>()
    }
}
impl URL {
    pub fn protocol(&self) -> USVString {
        self.inner.get("protocol").as_::<USVString>()
    }

    pub fn set_protocol(&mut self, value: USVString) {
        self.inner.set("protocol", value);
    }
}
impl URL {
    pub fn username(&self) -> USVString {
        self.inner.get("username").as_::<USVString>()
    }

    pub fn set_username(&mut self, value: USVString) {
        self.inner.set("username", value);
    }
}
impl URL {
    pub fn password(&self) -> USVString {
        self.inner.get("password").as_::<USVString>()
    }

    pub fn set_password(&mut self, value: USVString) {
        self.inner.set("password", value);
    }
}
impl URL {
    pub fn host(&self) -> USVString {
        self.inner.get("host").as_::<USVString>()
    }

    pub fn set_host(&mut self, value: USVString) {
        self.inner.set("host", value);
    }
}
impl URL {
    pub fn hostname(&self) -> USVString {
        self.inner.get("hostname").as_::<USVString>()
    }

    pub fn set_hostname(&mut self, value: USVString) {
        self.inner.set("hostname", value);
    }
}
impl URL {
    pub fn port(&self) -> USVString {
        self.inner.get("port").as_::<USVString>()
    }

    pub fn set_port(&mut self, value: USVString) {
        self.inner.set("port", value);
    }
}
impl URL {
    pub fn pathname(&self) -> USVString {
        self.inner.get("pathname").as_::<USVString>()
    }

    pub fn set_pathname(&mut self, value: USVString) {
        self.inner.set("pathname", value);
    }
}
impl URL {
    pub fn search(&self) -> USVString {
        self.inner.get("search").as_::<USVString>()
    }

    pub fn set_search(&mut self, value: USVString) {
        self.inner.set("search", value);
    }
}
impl URL {
    pub fn search_params(&self) -> URLSearchParams {
        self.inner.get("searchParams").as_::<URLSearchParams>()
    }
}
impl URL {
    pub fn hash(&self) -> USVString {
        self.inner.get("hash").as_::<USVString>()
    }

    pub fn set_hash(&mut self, value: USVString) {
        self.inner.set("hash", value);
    }
}
impl URL {
    pub fn to_json(&self) -> USVString {
        self.inner.call("toJSON", &[]).as_::<USVString>()
    }
}
impl URL {
    pub fn create_object_url(obj: Any) -> DOMString {
        emlite::Val::global("URL")
            .call("createObjectURL", &[obj.into()])
            .as_::<DOMString>()
    }
}
impl URL {
    pub fn revoke_object_url(url: DOMString) -> Undefined {
        emlite::Val::global("URL")
            .call("revokeObjectURL", &[url.into()])
            .as_::<Undefined>()
    }
}
