use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLPatternInit {
    inner: Any,
}
impl FromVal for URLPatternInit {
    fn from_val(v: &Any) -> Self {
        URLPatternInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for URLPatternInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for URLPatternInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for URLPatternInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for URLPatternInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<URLPatternInit> for Any {
    fn from(s: URLPatternInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&URLPatternInit> for Any {
    fn from(s: &URLPatternInit) -> Any {
        s.inner.clone()
    }
}

impl URLPatternInit {
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl URLPatternInit {
    pub fn username(&self) -> JsString {
        self.inner.get("username").as_::<JsString>()
    }

    pub fn set_username(&mut self, value: &JsString) {
        self.inner.set("username", value);
    }
}
impl URLPatternInit {
    pub fn password(&self) -> JsString {
        self.inner.get("password").as_::<JsString>()
    }

    pub fn set_password(&mut self, value: &JsString) {
        self.inner.set("password", value);
    }
}
impl URLPatternInit {
    pub fn hostname(&self) -> JsString {
        self.inner.get("hostname").as_::<JsString>()
    }

    pub fn set_hostname(&mut self, value: &JsString) {
        self.inner.set("hostname", value);
    }
}
impl URLPatternInit {
    pub fn port(&self) -> JsString {
        self.inner.get("port").as_::<JsString>()
    }

    pub fn set_port(&mut self, value: &JsString) {
        self.inner.set("port", value);
    }
}
impl URLPatternInit {
    pub fn pathname(&self) -> JsString {
        self.inner.get("pathname").as_::<JsString>()
    }

    pub fn set_pathname(&mut self, value: &JsString) {
        self.inner.set("pathname", value);
    }
}
impl URLPatternInit {
    pub fn search(&self) -> JsString {
        self.inner.get("search").as_::<JsString>()
    }

    pub fn set_search(&mut self, value: &JsString) {
        self.inner.set("search", value);
    }
}
impl URLPatternInit {
    pub fn hash(&self) -> JsString {
        self.inner.get("hash").as_::<JsString>()
    }

    pub fn set_hash(&mut self, value: &JsString) {
        self.inner.set("hash", value);
    }
}
impl URLPatternInit {
    pub fn base_url(&self) -> JsString {
        self.inner.get("baseURL").as_::<JsString>()
    }

    pub fn set_base_url(&mut self, value: &JsString) {
        self.inner.set("baseURL", value);
    }
}
