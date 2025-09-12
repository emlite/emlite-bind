use super::*;

/// The URL class.
/// [`URL`](https://developer.mozilla.org/en-US/docs/Web/API/URL)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URL {
    inner: Any,
}

impl FromVal for URL {
    fn from_val(v: &Any) -> Self {
        URL {
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

impl core::ops::Deref for URL {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for URL {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for URL {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for URL {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<URL> for Any {
    fn from(s: URL) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&URL> for Any {
    fn from(s: &URL) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(URL);

impl URL {
    /// Getter of the `href` attribute.
    /// [`URL.href`](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)
    pub fn href(&self) -> JsString {
        self.inner.get("href").as_::<JsString>()
    }

    /// Setter of the `href` attribute.
    /// [`URL.href`](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)
    pub fn set_href(&mut self, value: &JsString) {
        self.inner.set("href", value);
    }
}
impl URL {
    /// Getter of the `origin` attribute.
    /// [`URL.origin`](https://developer.mozilla.org/en-US/docs/Web/API/URL/origin)
    pub fn origin(&self) -> JsString {
        self.inner.get("origin").as_::<JsString>()
    }
}
impl URL {
    /// Getter of the `protocol` attribute.
    /// [`URL.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

    /// Setter of the `protocol` attribute.
    /// [`URL.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)
    pub fn set_protocol(&mut self, value: &JsString) {
        self.inner.set("protocol", value);
    }
}
impl URL {
    /// Getter of the `username` attribute.
    /// [`URL.username`](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)
    pub fn username(&self) -> JsString {
        self.inner.get("username").as_::<JsString>()
    }

    /// Setter of the `username` attribute.
    /// [`URL.username`](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)
    pub fn set_username(&mut self, value: &JsString) {
        self.inner.set("username", value);
    }
}
impl URL {
    /// Getter of the `password` attribute.
    /// [`URL.password`](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)
    pub fn password(&self) -> JsString {
        self.inner.get("password").as_::<JsString>()
    }

    /// Setter of the `password` attribute.
    /// [`URL.password`](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)
    pub fn set_password(&mut self, value: &JsString) {
        self.inner.set("password", value);
    }
}
impl URL {
    /// Getter of the `host` attribute.
    /// [`URL.host`](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)
    pub fn host(&self) -> JsString {
        self.inner.get("host").as_::<JsString>()
    }

    /// Setter of the `host` attribute.
    /// [`URL.host`](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)
    pub fn set_host(&mut self, value: &JsString) {
        self.inner.set("host", value);
    }
}
impl URL {
    /// Getter of the `hostname` attribute.
    /// [`URL.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)
    pub fn hostname(&self) -> JsString {
        self.inner.get("hostname").as_::<JsString>()
    }

    /// Setter of the `hostname` attribute.
    /// [`URL.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)
    pub fn set_hostname(&mut self, value: &JsString) {
        self.inner.set("hostname", value);
    }
}
impl URL {
    /// Getter of the `port` attribute.
    /// [`URL.port`](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)
    pub fn port(&self) -> JsString {
        self.inner.get("port").as_::<JsString>()
    }

    /// Setter of the `port` attribute.
    /// [`URL.port`](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)
    pub fn set_port(&mut self, value: &JsString) {
        self.inner.set("port", value);
    }
}
impl URL {
    /// Getter of the `pathname` attribute.
    /// [`URL.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn pathname(&self) -> JsString {
        self.inner.get("pathname").as_::<JsString>()
    }

    /// Setter of the `pathname` attribute.
    /// [`URL.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn set_pathname(&mut self, value: &JsString) {
        self.inner.set("pathname", value);
    }
}
impl URL {
    /// Getter of the `search` attribute.
    /// [`URL.search`](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    pub fn search(&self) -> JsString {
        self.inner.get("search").as_::<JsString>()
    }

    /// Setter of the `search` attribute.
    /// [`URL.search`](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    pub fn set_search(&mut self, value: &JsString) {
        self.inner.set("search", value);
    }
}
impl URL {
    /// Getter of the `searchParams` attribute.
    /// [`URL.searchParams`](https://developer.mozilla.org/en-US/docs/Web/API/URL/searchParams)
    pub fn search_params(&self) -> URLSearchParams {
        self.inner.get("searchParams").as_::<URLSearchParams>()
    }
}
impl URL {
    /// Getter of the `hash` attribute.
    /// [`URL.hash`](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    pub fn hash(&self) -> JsString {
        self.inner.get("hash").as_::<JsString>()
    }

    /// Setter of the `hash` attribute.
    /// [`URL.hash`](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    pub fn set_hash(&mut self, value: &JsString) {
        self.inner.set("hash", value);
    }
}

impl URL {
    /// The `new URL(..)` constructor, creating a new URL instance
    pub fn new0(url: &JsString) -> URL {
        Self {
            inner: Any::global("URL").new(&[url.into()]).as_::<Any>(),
        }
    }

    /// The `new URL(..)` constructor, creating a new URL instance
    pub fn new1(url: &JsString, base: &JsString) -> URL {
        Self {
            inner: Any::global("URL")
                .new(&[url.into(), base.into()])
                .as_::<Any>(),
        }
    }
}
impl URL {
    /// The parse method.
    /// [`URL.parse`](https://developer.mozilla.org/en-US/docs/Web/API/URL/parse)
    pub fn parse0(url: &JsString) -> URL {
        Any::global("URL").call("parse", &[url.into()]).as_::<URL>()
    }
    /// The parse method.
    /// [`URL.parse`](https://developer.mozilla.org/en-US/docs/Web/API/URL/parse)
    pub fn parse1(url: &JsString, base: &JsString) -> URL {
        Any::global("URL")
            .call("parse", &[url.into(), base.into()])
            .as_::<URL>()
    }
}
impl URL {
    /// The canParse method.
    /// [`URL.canParse`](https://developer.mozilla.org/en-US/docs/Web/API/URL/canParse)
    pub fn can_parse0(url: &JsString) -> bool {
        Any::global("URL")
            .call("canParse", &[url.into()])
            .as_::<bool>()
    }
    /// The canParse method.
    /// [`URL.canParse`](https://developer.mozilla.org/en-US/docs/Web/API/URL/canParse)
    pub fn can_parse1(url: &JsString, base: &JsString) -> bool {
        Any::global("URL")
            .call("canParse", &[url.into(), base.into()])
            .as_::<bool>()
    }
}
impl URL {
    /// The toJSON method.
    /// [`URL.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/URL/toJSON)
    pub fn to_json(&self) -> JsString {
        self.inner.call("toJSON", &[]).as_::<JsString>()
    }
}
impl URL {
    /// The createObjectURL method.
    /// [`URL.createObjectURL`](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL)
    pub fn create_object_url(obj: &Any) -> JsString {
        Any::global("URL")
            .call("createObjectURL", &[obj.into()])
            .as_::<JsString>()
    }
}
impl URL {
    /// The revokeObjectURL method.
    /// [`URL.revokeObjectURL`](https://developer.mozilla.org/en-US/docs/Web/API/URL/revokeObjectURL)
    pub fn revoke_object_url(url: &JsString) -> Undefined {
        Any::global("URL")
            .call("revokeObjectURL", &[url.into()])
            .as_::<Undefined>()
    }
}
