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
    /// The `new URL(..)` constructor, creating a new URL instance
    pub fn new0(url: &str) -> URL {
        Self {
            inner: Any::global("URL").new(&[url.into()]).as_::<Any>(),
        }
    }

    /// The `new URL(..)` constructor, creating a new URL instance
    pub fn new1(url: &str, base: &str) -> URL {
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
    pub fn parse0(url: &str) -> URL {
        Any::global("URL").call("parse", &[url.into()]).as_::<URL>()
    }
    /// The parse method.
    /// [`URL.parse`](https://developer.mozilla.org/en-US/docs/Web/API/URL/parse)
    pub fn parse1(url: &str, base: &str) -> URL {
        Any::global("URL")
            .call("parse", &[url.into(), base.into()])
            .as_::<URL>()
    }
}
impl URL {
    /// The canParse method.
    /// [`URL.canParse`](https://developer.mozilla.org/en-US/docs/Web/API/URL/canParse)
    pub fn can_parse0(url: &str) -> bool {
        Any::global("URL")
            .call("canParse", &[url.into()])
            .as_::<bool>()
    }
    /// The canParse method.
    /// [`URL.canParse`](https://developer.mozilla.org/en-US/docs/Web/API/URL/canParse)
    pub fn can_parse1(url: &str, base: &str) -> bool {
        Any::global("URL")
            .call("canParse", &[url.into(), base.into()])
            .as_::<bool>()
    }
}
impl URL {
    /// Getter of the `href` attribute.
    /// [`URL.href`](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)
    pub fn href(&self) -> String {
        self.inner.get("href").as_::<String>()
    }

    /// Setter of the `href` attribute.
    /// [`URL.href`](https://developer.mozilla.org/en-US/docs/Web/API/URL/href)
    pub fn set_href(&mut self, value: &str) {
        self.inner.set("href", value);
    }
}
impl URL {
    /// Getter of the `origin` attribute.
    /// [`URL.origin`](https://developer.mozilla.org/en-US/docs/Web/API/URL/origin)
    pub fn origin(&self) -> String {
        self.inner.get("origin").as_::<String>()
    }
}
impl URL {
    /// Getter of the `protocol` attribute.
    /// [`URL.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)
    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }

    /// Setter of the `protocol` attribute.
    /// [`URL.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/URL/protocol)
    pub fn set_protocol(&mut self, value: &str) {
        self.inner.set("protocol", value);
    }
}
impl URL {
    /// Getter of the `username` attribute.
    /// [`URL.username`](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)
    pub fn username(&self) -> String {
        self.inner.get("username").as_::<String>()
    }

    /// Setter of the `username` attribute.
    /// [`URL.username`](https://developer.mozilla.org/en-US/docs/Web/API/URL/username)
    pub fn set_username(&mut self, value: &str) {
        self.inner.set("username", value);
    }
}
impl URL {
    /// Getter of the `password` attribute.
    /// [`URL.password`](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)
    pub fn password(&self) -> String {
        self.inner.get("password").as_::<String>()
    }

    /// Setter of the `password` attribute.
    /// [`URL.password`](https://developer.mozilla.org/en-US/docs/Web/API/URL/password)
    pub fn set_password(&mut self, value: &str) {
        self.inner.set("password", value);
    }
}
impl URL {
    /// Getter of the `host` attribute.
    /// [`URL.host`](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)
    pub fn host(&self) -> String {
        self.inner.get("host").as_::<String>()
    }

    /// Setter of the `host` attribute.
    /// [`URL.host`](https://developer.mozilla.org/en-US/docs/Web/API/URL/host)
    pub fn set_host(&mut self, value: &str) {
        self.inner.set("host", value);
    }
}
impl URL {
    /// Getter of the `hostname` attribute.
    /// [`URL.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)
    pub fn hostname(&self) -> String {
        self.inner.get("hostname").as_::<String>()
    }

    /// Setter of the `hostname` attribute.
    /// [`URL.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/URL/hostname)
    pub fn set_hostname(&mut self, value: &str) {
        self.inner.set("hostname", value);
    }
}
impl URL {
    /// Getter of the `port` attribute.
    /// [`URL.port`](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)
    pub fn port(&self) -> String {
        self.inner.get("port").as_::<String>()
    }

    /// Setter of the `port` attribute.
    /// [`URL.port`](https://developer.mozilla.org/en-US/docs/Web/API/URL/port)
    pub fn set_port(&mut self, value: &str) {
        self.inner.set("port", value);
    }
}
impl URL {
    /// Getter of the `pathname` attribute.
    /// [`URL.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn pathname(&self) -> String {
        self.inner.get("pathname").as_::<String>()
    }

    /// Setter of the `pathname` attribute.
    /// [`URL.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/URL/pathname)
    pub fn set_pathname(&mut self, value: &str) {
        self.inner.set("pathname", value);
    }
}
impl URL {
    /// Getter of the `search` attribute.
    /// [`URL.search`](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    pub fn search(&self) -> String {
        self.inner.get("search").as_::<String>()
    }

    /// Setter of the `search` attribute.
    /// [`URL.search`](https://developer.mozilla.org/en-US/docs/Web/API/URL/search)
    pub fn set_search(&mut self, value: &str) {
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
    pub fn hash(&self) -> String {
        self.inner.get("hash").as_::<String>()
    }

    /// Setter of the `hash` attribute.
    /// [`URL.hash`](https://developer.mozilla.org/en-US/docs/Web/API/URL/hash)
    pub fn set_hash(&mut self, value: &str) {
        self.inner.set("hash", value);
    }
}
impl URL {
    /// The toJSON method.
    /// [`URL.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/URL/toJSON)
    pub fn to_json(&self) -> String {
        self.inner.call("toJSON", &[]).as_::<String>()
    }
}
impl URL {
    /// The createObjectURL method.
    /// [`URL.createObjectURL`](https://developer.mozilla.org/en-US/docs/Web/API/URL/createObjectURL)
    pub fn create_object_url(obj: &Any) -> String {
        Any::global("URL")
            .call("createObjectURL", &[obj.into()])
            .as_::<String>()
    }
}
impl URL {
    /// The revokeObjectURL method.
    /// [`URL.revokeObjectURL`](https://developer.mozilla.org/en-US/docs/Web/API/URL/revokeObjectURL)
    pub fn revoke_object_url(url: &str) -> Undefined {
        Any::global("URL")
            .call("revokeObjectURL", &[url.into()])
            .as_::<Undefined>()
    }
}
