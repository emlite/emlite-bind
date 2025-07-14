use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<URL> for emlite::Val {
    fn from(s: URL) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl URL {
    pub fn new0(url: jsbind::USVString) -> URL {
        Self {
            inner: emlite::Val::global("URL")
                .new(&[url.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(url: jsbind::USVString, base: jsbind::USVString) -> URL {
        Self {
            inner: emlite::Val::global("URL")
                .new(&[url.into(), base.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl URL {
    pub fn parse0(url: jsbind::USVString) -> URL {
        emlite::Val::global("url")
            .call("parse", &[url.into()])
            .as_::<URL>()
    }

    pub fn parse1(url: jsbind::USVString, base: jsbind::USVString) -> URL {
        emlite::Val::global("url")
            .call("parse", &[url.into(), base.into()])
            .as_::<URL>()
    }
}
impl URL {
    pub fn can_parse0(url: jsbind::USVString) -> bool {
        emlite::Val::global("url")
            .call("canParse", &[url.into()])
            .as_::<bool>()
    }

    pub fn can_parse1(url: jsbind::USVString, base: jsbind::USVString) -> bool {
        emlite::Val::global("url")
            .call("canParse", &[url.into(), base.into()])
            .as_::<bool>()
    }
}
impl URL {
    pub fn href(&self) -> jsbind::USVString {
        self.inner.get("href").as_::<jsbind::USVString>()
    }

    pub fn set_href(&mut self, value: jsbind::USVString) {
        self.inner.set("href", value);
    }
}
impl URL {
    pub fn origin(&self) -> jsbind::USVString {
        self.inner.get("origin").as_::<jsbind::USVString>()
    }
}
impl URL {
    pub fn protocol(&self) -> jsbind::USVString {
        self.inner.get("protocol").as_::<jsbind::USVString>()
    }

    pub fn set_protocol(&mut self, value: jsbind::USVString) {
        self.inner.set("protocol", value);
    }
}
impl URL {
    pub fn username(&self) -> jsbind::USVString {
        self.inner.get("username").as_::<jsbind::USVString>()
    }

    pub fn set_username(&mut self, value: jsbind::USVString) {
        self.inner.set("username", value);
    }
}
impl URL {
    pub fn password(&self) -> jsbind::USVString {
        self.inner.get("password").as_::<jsbind::USVString>()
    }

    pub fn set_password(&mut self, value: jsbind::USVString) {
        self.inner.set("password", value);
    }
}
impl URL {
    pub fn host(&self) -> jsbind::USVString {
        self.inner.get("host").as_::<jsbind::USVString>()
    }

    pub fn set_host(&mut self, value: jsbind::USVString) {
        self.inner.set("host", value);
    }
}
impl URL {
    pub fn hostname(&self) -> jsbind::USVString {
        self.inner.get("hostname").as_::<jsbind::USVString>()
    }

    pub fn set_hostname(&mut self, value: jsbind::USVString) {
        self.inner.set("hostname", value);
    }
}
impl URL {
    pub fn port(&self) -> jsbind::USVString {
        self.inner.get("port").as_::<jsbind::USVString>()
    }

    pub fn set_port(&mut self, value: jsbind::USVString) {
        self.inner.set("port", value);
    }
}
impl URL {
    pub fn pathname(&self) -> jsbind::USVString {
        self.inner.get("pathname").as_::<jsbind::USVString>()
    }

    pub fn set_pathname(&mut self, value: jsbind::USVString) {
        self.inner.set("pathname", value);
    }
}
impl URL {
    pub fn search(&self) -> jsbind::USVString {
        self.inner.get("search").as_::<jsbind::USVString>()
    }

    pub fn set_search(&mut self, value: jsbind::USVString) {
        self.inner.set("search", value);
    }
}
impl URL {
    pub fn search_params(&self) -> URLSearchParams {
        self.inner.get("searchParams").as_::<URLSearchParams>()
    }
}
impl URL {
    pub fn hash(&self) -> jsbind::USVString {
        self.inner.get("hash").as_::<jsbind::USVString>()
    }

    pub fn set_hash(&mut self, value: jsbind::USVString) {
        self.inner.set("hash", value);
    }
}
impl URL {
    pub fn to_json(&self) -> jsbind::USVString {
        self.inner.call("toJSON", &[]).as_::<jsbind::USVString>()
    }
}
impl URL {
    pub fn create_object_url(obj: jsbind::Any) -> jsbind::DOMString {
        emlite::Val::global("url")
            .call("createObjectURL", &[obj.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl URL {
    pub fn revoke_object_url(url: jsbind::DOMString) -> jsbind::Undefined {
        emlite::Val::global("url")
            .call("revokeObjectURL", &[url.into()])
            .as_::<jsbind::Undefined>()
    }
}
