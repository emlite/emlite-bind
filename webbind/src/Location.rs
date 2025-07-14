use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<Location> for emlite::Val {
    fn from(s: Location) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Location {
    pub fn href(&self) -> jsbind::USVString {
        self.inner.get("href").as_::<jsbind::USVString>()
    }

    pub fn set_href(&mut self, value: jsbind::USVString) {
        self.inner.set("href", value);
    }
}
impl Location {
    pub fn origin(&self) -> jsbind::USVString {
        self.inner.get("origin").as_::<jsbind::USVString>()
    }
}
impl Location {
    pub fn protocol(&self) -> jsbind::USVString {
        self.inner.get("protocol").as_::<jsbind::USVString>()
    }

    pub fn set_protocol(&mut self, value: jsbind::USVString) {
        self.inner.set("protocol", value);
    }
}
impl Location {
    pub fn host(&self) -> jsbind::USVString {
        self.inner.get("host").as_::<jsbind::USVString>()
    }

    pub fn set_host(&mut self, value: jsbind::USVString) {
        self.inner.set("host", value);
    }
}
impl Location {
    pub fn hostname(&self) -> jsbind::USVString {
        self.inner.get("hostname").as_::<jsbind::USVString>()
    }

    pub fn set_hostname(&mut self, value: jsbind::USVString) {
        self.inner.set("hostname", value);
    }
}
impl Location {
    pub fn port(&self) -> jsbind::USVString {
        self.inner.get("port").as_::<jsbind::USVString>()
    }

    pub fn set_port(&mut self, value: jsbind::USVString) {
        self.inner.set("port", value);
    }
}
impl Location {
    pub fn pathname(&self) -> jsbind::USVString {
        self.inner.get("pathname").as_::<jsbind::USVString>()
    }

    pub fn set_pathname(&mut self, value: jsbind::USVString) {
        self.inner.set("pathname", value);
    }
}
impl Location {
    pub fn search(&self) -> jsbind::USVString {
        self.inner.get("search").as_::<jsbind::USVString>()
    }

    pub fn set_search(&mut self, value: jsbind::USVString) {
        self.inner.set("search", value);
    }
}
impl Location {
    pub fn hash(&self) -> jsbind::USVString {
        self.inner.get("hash").as_::<jsbind::USVString>()
    }

    pub fn set_hash(&mut self, value: jsbind::USVString) {
        self.inner.set("hash", value);
    }
}
impl Location {
    pub fn assign(&self, url: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("assign", &[url.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Location {
    pub fn replace(&self, url: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("replace", &[url.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Location {
    pub fn reload(&self) -> jsbind::Undefined {
        self.inner.call("reload", &[]).as_::<jsbind::Undefined>()
    }
}
impl Location {
    pub fn ancestor_origins(&self) -> DOMStringList {
        self.inner.get("ancestorOrigins").as_::<DOMStringList>()
    }
}
