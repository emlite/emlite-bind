use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLPatternResult {
    inner: Any,
}
impl FromVal for URLPatternResult {
    fn from_val(v: &Any) -> Self {
        URLPatternResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for URLPatternResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for URLPatternResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for URLPatternResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for URLPatternResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<URLPatternResult> for Any {
    fn from(s: URLPatternResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&URLPatternResult> for Any {
    fn from(s: &URLPatternResult) -> Any {
        s.inner.clone()
    }
}

impl URLPatternResult {
    pub fn inputs(&self) -> Sequence<Any> {
        self.inner.get("inputs").as_::<Sequence<Any>>()
    }

    pub fn set_inputs(&mut self, value: &Sequence<Any>) {
        self.inner.set("inputs", value);
    }
}
impl URLPatternResult {
    pub fn protocol(&self) -> Any {
        self.inner.get("protocol").as_::<Any>()
    }

    pub fn set_protocol(&mut self, value: &Any) {
        self.inner.set("protocol", value);
    }
}
impl URLPatternResult {
    pub fn username(&self) -> Any {
        self.inner.get("username").as_::<Any>()
    }

    pub fn set_username(&mut self, value: &Any) {
        self.inner.set("username", value);
    }
}
impl URLPatternResult {
    pub fn password(&self) -> Any {
        self.inner.get("password").as_::<Any>()
    }

    pub fn set_password(&mut self, value: &Any) {
        self.inner.set("password", value);
    }
}
impl URLPatternResult {
    pub fn hostname(&self) -> Any {
        self.inner.get("hostname").as_::<Any>()
    }

    pub fn set_hostname(&mut self, value: &Any) {
        self.inner.set("hostname", value);
    }
}
impl URLPatternResult {
    pub fn port(&self) -> Any {
        self.inner.get("port").as_::<Any>()
    }

    pub fn set_port(&mut self, value: &Any) {
        self.inner.set("port", value);
    }
}
impl URLPatternResult {
    pub fn pathname(&self) -> Any {
        self.inner.get("pathname").as_::<Any>()
    }

    pub fn set_pathname(&mut self, value: &Any) {
        self.inner.set("pathname", value);
    }
}
impl URLPatternResult {
    pub fn search(&self) -> Any {
        self.inner.get("search").as_::<Any>()
    }

    pub fn set_search(&mut self, value: &Any) {
        self.inner.set("search", value);
    }
}
impl URLPatternResult {
    pub fn hash(&self) -> Any {
        self.inner.get("hash").as_::<Any>()
    }

    pub fn set_hash(&mut self, value: &Any) {
        self.inner.set("hash", value);
    }
}
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
    pub fn new2(input: &Any, options: &Any) -> URLPattern {
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
    pub fn test2(&self, input: &Any, base_url: &USVString) -> bool {
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
    pub fn exec2(&self, input: &Any, base_url: &USVString) -> URLPatternResult {
        self.inner
            .call("exec", &[input.into(), base_url.into()])
            .as_::<URLPatternResult>()
    }
}
impl URLPattern {
    /// Getter of the `protocol` attribute.
    /// [`URLPattern.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/protocol)
    pub fn protocol(&self) -> USVString {
        self.inner.get("protocol").as_::<USVString>()
    }
}
impl URLPattern {
    /// Getter of the `username` attribute.
    /// [`URLPattern.username`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/username)
    pub fn username(&self) -> USVString {
        self.inner.get("username").as_::<USVString>()
    }
}
impl URLPattern {
    /// Getter of the `password` attribute.
    /// [`URLPattern.password`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/password)
    pub fn password(&self) -> USVString {
        self.inner.get("password").as_::<USVString>()
    }
}
impl URLPattern {
    /// Getter of the `hostname` attribute.
    /// [`URLPattern.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/hostname)
    pub fn hostname(&self) -> USVString {
        self.inner.get("hostname").as_::<USVString>()
    }
}
impl URLPattern {
    /// Getter of the `port` attribute.
    /// [`URLPattern.port`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/port)
    pub fn port(&self) -> USVString {
        self.inner.get("port").as_::<USVString>()
    }
}
impl URLPattern {
    /// Getter of the `pathname` attribute.
    /// [`URLPattern.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/pathname)
    pub fn pathname(&self) -> USVString {
        self.inner.get("pathname").as_::<USVString>()
    }
}
impl URLPattern {
    /// Getter of the `search` attribute.
    /// [`URLPattern.search`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/search)
    pub fn search(&self) -> USVString {
        self.inner.get("search").as_::<USVString>()
    }
}
impl URLPattern {
    /// Getter of the `hash` attribute.
    /// [`URLPattern.hash`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/hash)
    pub fn hash(&self) -> USVString {
        self.inner.get("hash").as_::<USVString>()
    }
}
impl URLPattern {
    /// Getter of the `hasRegExpGroups` attribute.
    /// [`URLPattern.hasRegExpGroups`](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern/hasRegExpGroups)
    pub fn has_reg_exp_groups(&self) -> bool {
        self.inner.get("hasRegExpGroups").as_::<bool>()
    }
}
