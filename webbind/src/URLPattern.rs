use super::*;

/// The URLPattern class.
/// [`URLPattern`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLPattern {
    inner: Any,
}

impl FromVal for URLPattern {
    fn from_val(v: &Any) -> Self {
        URLPattern {
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

impl core::ops::Deref for URLPattern {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for URLPattern {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for URLPattern {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for URLPattern {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<URLPattern> for Any {
    fn from(s: URLPattern) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&URLPattern> for Any {
    fn from(s: &URLPattern) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(URLPattern);

impl URLPattern {
    /// The `new URLPattern(..)` constructor, creating a new URLPattern instance
    pub fn new0() -> URLPattern {
        Self {
            inner: Any::global("URLPattern").new(&[]).as_::<Any>(),
        }
    }

    /// The `new URLPattern(..)` constructor, creating a new URLPattern instance
    pub fn new1(input: &Any) -> URLPattern {
        Self {
            inner: Any::global("URLPattern").new(&[input.into()]).as_::<Any>(),
        }
    }

    /// The `new URLPattern(..)` constructor, creating a new URLPattern instance
    pub fn new2(input: &Any, options: &URLPatternOptions) -> URLPattern {
        Self {
            inner: Any::global("URLPattern")
                .new(&[input.into(), options.into()])
                .as_::<Any>(),
        }
    }
}
impl URLPattern {
    /// The test method.
    /// [`URLPattern.test`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/test)
    pub fn test0(&self) -> bool {
        self.inner.call("test", &[]).as_::<bool>()
    }
    /// The test method.
    /// [`URLPattern.test`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/test)
    pub fn test1(&self, input: &Any) -> bool {
        self.inner.call("test", &[input.into()]).as_::<bool>()
    }
    /// The test method.
    /// [`URLPattern.test`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/test)
    pub fn test2(&self, input: &Any, base_url: &JsString) -> bool {
        self.inner
            .call("test", &[input.into(), base_url.into()])
            .as_::<bool>()
    }
}
impl URLPattern {
    /// The exec method.
    /// [`URLPattern.exec`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/exec)
    pub fn exec0(&self) -> URLPatternResult {
        self.inner.call("exec", &[]).as_::<URLPatternResult>()
    }
    /// The exec method.
    /// [`URLPattern.exec`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/exec)
    pub fn exec1(&self, input: &Any) -> URLPatternResult {
        self.inner
            .call("exec", &[input.into()])
            .as_::<URLPatternResult>()
    }
    /// The exec method.
    /// [`URLPattern.exec`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/exec)
    pub fn exec2(&self, input: &Any, base_url: &JsString) -> URLPatternResult {
        self.inner
            .call("exec", &[input.into(), base_url.into()])
            .as_::<URLPatternResult>()
    }
}
impl URLPattern {
    /// Getter of the `protocol` attribute.
    /// [`URLPattern.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/protocol)
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }
}
impl URLPattern {
    /// Getter of the `username` attribute.
    /// [`URLPattern.username`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/username)
    pub fn username(&self) -> JsString {
        self.inner.get("username").as_::<JsString>()
    }
}
impl URLPattern {
    /// Getter of the `password` attribute.
    /// [`URLPattern.password`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/password)
    pub fn password(&self) -> JsString {
        self.inner.get("password").as_::<JsString>()
    }
}
impl URLPattern {
    /// Getter of the `hostname` attribute.
    /// [`URLPattern.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/hostname)
    pub fn hostname(&self) -> JsString {
        self.inner.get("hostname").as_::<JsString>()
    }
}
impl URLPattern {
    /// Getter of the `port` attribute.
    /// [`URLPattern.port`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/port)
    pub fn port(&self) -> JsString {
        self.inner.get("port").as_::<JsString>()
    }
}
impl URLPattern {
    /// Getter of the `pathname` attribute.
    /// [`URLPattern.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/pathname)
    pub fn pathname(&self) -> JsString {
        self.inner.get("pathname").as_::<JsString>()
    }
}
impl URLPattern {
    /// Getter of the `search` attribute.
    /// [`URLPattern.search`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/search)
    pub fn search(&self) -> JsString {
        self.inner.get("search").as_::<JsString>()
    }
}
impl URLPattern {
    /// Getter of the `hash` attribute.
    /// [`URLPattern.hash`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/hash)
    pub fn hash(&self) -> JsString {
        self.inner.get("hash").as_::<JsString>()
    }
}
impl URLPattern {
    /// Getter of the `hasRegExpGroups` attribute.
    /// [`URLPattern.hasRegExpGroups`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/hasRegExpGroups)
    pub fn has_reg_exp_groups(&self) -> bool {
        self.inner.get("hasRegExpGroups").as_::<bool>()
    }
}
