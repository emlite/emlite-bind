use super::*;

/// The Location class.
/// [`Location`](https://developer.mozilla.org/en-US/docs/Web/API/Location)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Location {
    inner: Any,
}
impl FromVal for Location {
    fn from_val(v: &Any) -> Self {
        Location {
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
impl core::ops::Deref for Location {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Location {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Location {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Location {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Location> for Any {
    fn from(s: Location) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Location> for Any {
    fn from(s: &Location) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Location);

impl Location {
    /// Getter of the `href` attribute.
    /// [`Location.href`](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }

    /// Setter of the `href` attribute.
    /// [`Location.href`](https://developer.mozilla.org/en-US/docs/Web/API/Location/href)
    pub fn set_href(&mut self, value: &USVString) {
        self.inner.set("href", value);
    }
}
impl Location {
    /// Getter of the `origin` attribute.
    /// [`Location.origin`](https://developer.mozilla.org/en-US/docs/Web/API/Location/origin)
    pub fn origin(&self) -> USVString {
        self.inner.get("origin").as_::<USVString>()
    }
}
impl Location {
    /// Getter of the `protocol` attribute.
    /// [`Location.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)
    pub fn protocol(&self) -> USVString {
        self.inner.get("protocol").as_::<USVString>()
    }

    /// Setter of the `protocol` attribute.
    /// [`Location.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/Location/protocol)
    pub fn set_protocol(&mut self, value: &USVString) {
        self.inner.set("protocol", value);
    }
}
impl Location {
    /// Getter of the `host` attribute.
    /// [`Location.host`](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)
    pub fn host(&self) -> USVString {
        self.inner.get("host").as_::<USVString>()
    }

    /// Setter of the `host` attribute.
    /// [`Location.host`](https://developer.mozilla.org/en-US/docs/Web/API/Location/host)
    pub fn set_host(&mut self, value: &USVString) {
        self.inner.set("host", value);
    }
}
impl Location {
    /// Getter of the `hostname` attribute.
    /// [`Location.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)
    pub fn hostname(&self) -> USVString {
        self.inner.get("hostname").as_::<USVString>()
    }

    /// Setter of the `hostname` attribute.
    /// [`Location.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/Location/hostname)
    pub fn set_hostname(&mut self, value: &USVString) {
        self.inner.set("hostname", value);
    }
}
impl Location {
    /// Getter of the `port` attribute.
    /// [`Location.port`](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)
    pub fn port(&self) -> USVString {
        self.inner.get("port").as_::<USVString>()
    }

    /// Setter of the `port` attribute.
    /// [`Location.port`](https://developer.mozilla.org/en-US/docs/Web/API/Location/port)
    pub fn set_port(&mut self, value: &USVString) {
        self.inner.set("port", value);
    }
}
impl Location {
    /// Getter of the `pathname` attribute.
    /// [`Location.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)
    pub fn pathname(&self) -> USVString {
        self.inner.get("pathname").as_::<USVString>()
    }

    /// Setter of the `pathname` attribute.
    /// [`Location.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/Location/pathname)
    pub fn set_pathname(&mut self, value: &USVString) {
        self.inner.set("pathname", value);
    }
}
impl Location {
    /// Getter of the `search` attribute.
    /// [`Location.search`](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)
    pub fn search(&self) -> USVString {
        self.inner.get("search").as_::<USVString>()
    }

    /// Setter of the `search` attribute.
    /// [`Location.search`](https://developer.mozilla.org/en-US/docs/Web/API/Location/search)
    pub fn set_search(&mut self, value: &USVString) {
        self.inner.set("search", value);
    }
}
impl Location {
    /// Getter of the `hash` attribute.
    /// [`Location.hash`](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)
    pub fn hash(&self) -> USVString {
        self.inner.get("hash").as_::<USVString>()
    }

    /// Setter of the `hash` attribute.
    /// [`Location.hash`](https://developer.mozilla.org/en-US/docs/Web/API/Location/hash)
    pub fn set_hash(&mut self, value: &USVString) {
        self.inner.set("hash", value);
    }
}
impl Location {
    /// The assign method.
    /// [`Location.assign`](https://developer.mozilla.org/en-US/docs/Web/API/Location/assign)
    pub fn assign(&self, url: &USVString) -> Undefined {
        self.inner.call("assign", &[url.into()]).as_::<Undefined>()
    }
}
impl Location {
    /// The replace method.
    /// [`Location.replace`](https://developer.mozilla.org/en-US/docs/Web/API/Location/replace)
    pub fn replace(&self, url: &USVString) -> Undefined {
        self.inner.call("replace", &[url.into()]).as_::<Undefined>()
    }
}
impl Location {
    /// The reload method.
    /// [`Location.reload`](https://developer.mozilla.org/en-US/docs/Web/API/Location/reload)
    pub fn reload(&self) -> Undefined {
        self.inner.call("reload", &[]).as_::<Undefined>()
    }
}
impl Location {
    /// Getter of the `ancestorOrigins` attribute.
    /// [`Location.ancestorOrigins`](https://developer.mozilla.org/en-US/docs/Web/API/Location/ancestorOrigins)
    pub fn ancestor_origins(&self) -> DOMStringList {
        self.inner.get("ancestorOrigins").as_::<DOMStringList>()
    }
}
