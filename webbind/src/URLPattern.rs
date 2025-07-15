use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLPatternResult {
    inner: emlite::Val,
}
impl FromVal for URLPatternResult {
    fn from_val(v: &emlite::Val) -> Self {
        URLPatternResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for URLPatternResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for URLPatternResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for URLPatternResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for URLPatternResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<URLPatternResult> for emlite::Val {
    fn from(s: URLPatternResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&URLPatternResult> for emlite::Val {
    fn from(s: &URLPatternResult) -> emlite::Val {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLPattern {
    inner: emlite::Val,
}
impl FromVal for URLPattern {
    fn from_val(v: &emlite::Val) -> Self {
        URLPattern {
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
impl core::ops::Deref for URLPattern {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for URLPattern {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for URLPattern {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for URLPattern {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<URLPattern> for emlite::Val {
    fn from(s: URLPattern) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&URLPattern> for emlite::Val {
    fn from(s: &URLPattern) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(URLPattern);

impl URLPattern {
    pub fn new0() -> URLPattern {
        Self {
            inner: emlite::Val::global("URLPattern")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(input: &Any) -> URLPattern {
        Self {
            inner: emlite::Val::global("URLPattern")
                .new(&[input.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(input: &Any, options: &Any) -> URLPattern {
        Self {
            inner: emlite::Val::global("URLPattern")
                .new(&[input.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl URLPattern {
    pub fn test0(&self) -> bool {
        self.inner.call("test", &[]).as_::<bool>()
    }

    pub fn test1(&self, input: &Any) -> bool {
        self.inner.call("test", &[input.into()]).as_::<bool>()
    }

    pub fn test2(&self, input: &Any, base_url: &str) -> bool {
        self.inner
            .call("test", &[input.into(), base_url.into()])
            .as_::<bool>()
    }
}
impl URLPattern {
    pub fn exec0(&self) -> URLPatternResult {
        self.inner.call("exec", &[]).as_::<URLPatternResult>()
    }

    pub fn exec1(&self, input: &Any) -> URLPatternResult {
        self.inner
            .call("exec", &[input.into()])
            .as_::<URLPatternResult>()
    }

    pub fn exec2(&self, input: &Any, base_url: &str) -> URLPatternResult {
        self.inner
            .call("exec", &[input.into(), base_url.into()])
            .as_::<URLPatternResult>()
    }
}
impl URLPattern {
    pub fn protocol(&self) -> String {
        self.inner.get("protocol").as_::<String>()
    }
}
impl URLPattern {
    pub fn username(&self) -> String {
        self.inner.get("username").as_::<String>()
    }
}
impl URLPattern {
    pub fn password(&self) -> String {
        self.inner.get("password").as_::<String>()
    }
}
impl URLPattern {
    pub fn hostname(&self) -> String {
        self.inner.get("hostname").as_::<String>()
    }
}
impl URLPattern {
    pub fn port(&self) -> String {
        self.inner.get("port").as_::<String>()
    }
}
impl URLPattern {
    pub fn pathname(&self) -> String {
        self.inner.get("pathname").as_::<String>()
    }
}
impl URLPattern {
    pub fn search(&self) -> String {
        self.inner.get("search").as_::<String>()
    }
}
impl URLPattern {
    pub fn hash(&self) -> String {
        self.inner.get("hash").as_::<String>()
    }
}
impl URLPattern {
    pub fn has_reg_exp_groups(&self) -> bool {
        self.inner.get("hasRegExpGroups").as_::<bool>()
    }
}
