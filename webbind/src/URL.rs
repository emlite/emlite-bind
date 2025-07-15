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
impl From<&URL> for emlite::Val {
    fn from(s: &URL) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(URL);

impl URL {
    pub fn new0(url: &str) -> URL {
        Self {
            inner: emlite::Val::global("URL")
                .new(&[url.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(url: &str, base: &str) -> URL {
        Self {
            inner: emlite::Val::global("URL")
                .new(&[url.into(), base.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl URL {
    pub fn parse0(url: &str) -> URL {
        emlite::Val::global("URL")
            .call("parse", &[url.into()])
            .as_::<URL>()
    }

    pub fn parse1(url: &str, base: &str) -> URL {
        emlite::Val::global("URL")
            .call("parse", &[url.into(), base.into()])
            .as_::<URL>()
    }
}
impl URL {
    pub fn can_parse0(url: &str) -> bool {
        emlite::Val::global("URL")
            .call("canParse", &[url.into()])
            .as_::<bool>()
    }

    pub fn can_parse1(url: &str, base: &str) -> bool {
        emlite::Val::global("URL")
            .call("canParse", &[url.into(), base.into()])
            .as_::<bool>()
    }
}
impl URL {
    pub fn href(&self) -> String {
        self.inner.get("href").as_::<String>()
    }

    pub fn set_href(&mut self, value: &str) {
        self.inner.set("href", value);
    }
}
impl URL {
    pub fn origin(&self) -> String {
        self.inner.get("origin").as_::<String>()
    }
}
impl URL {
    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }

    pub fn set_protocol(&mut self, value: &str) {
        self.inner.set("protocol", value);
    }
}
impl URL {
    pub fn username(&self) -> String {
        self.inner.get("username").as_::<String>()
    }

    pub fn set_username(&mut self, value: &str) {
        self.inner.set("username", value);
    }
}
impl URL {
    pub fn password(&self) -> String {
        self.inner.get("password").as_::<String>()
    }

    pub fn set_password(&mut self, value: &str) {
        self.inner.set("password", value);
    }
}
impl URL {
    pub fn host(&self) -> String {
        self.inner.get("host").as_::<String>()
    }

    pub fn set_host(&mut self, value: &str) {
        self.inner.set("host", value);
    }
}
impl URL {
    pub fn hostname(&self) -> String {
        self.inner.get("hostname").as_::<String>()
    }

    pub fn set_hostname(&mut self, value: &str) {
        self.inner.set("hostname", value);
    }
}
impl URL {
    pub fn port(&self) -> String {
        self.inner.get("port").as_::<String>()
    }

    pub fn set_port(&mut self, value: &str) {
        self.inner.set("port", value);
    }
}
impl URL {
    pub fn pathname(&self) -> String {
        self.inner.get("pathname").as_::<String>()
    }

    pub fn set_pathname(&mut self, value: &str) {
        self.inner.set("pathname", value);
    }
}
impl URL {
    pub fn search(&self) -> String {
        self.inner.get("search").as_::<String>()
    }

    pub fn set_search(&mut self, value: &str) {
        self.inner.set("search", value);
    }
}
impl URL {
    pub fn search_params(&self) -> URLSearchParams {
        self.inner.get("searchParams").as_::<URLSearchParams>()
    }
}
impl URL {
    pub fn hash(&self) -> String {
        self.inner.get("hash").as_::<String>()
    }

    pub fn set_hash(&mut self, value: &str) {
        self.inner.set("hash", value);
    }
}
impl URL {
    pub fn to_json(&self) -> String {
        self.inner.call("toJSON", &[]).as_::<String>()
    }
}
impl URL {
    pub fn create_object_url(obj: &Any) -> String {
        emlite::Val::global("URL")
            .call("createObjectURL", &[obj.into()])
            .as_::<String>()
    }
}
impl URL {
    pub fn revoke_object_url(url: &str) -> Undefined {
        emlite::Val::global("URL")
            .call("revokeObjectURL", &[url.into()])
            .as_::<Undefined>()
    }
}
