use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<URLPatternResult> for emlite::Val {
    fn from(s: URLPatternResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl URLPatternResult {
    pub fn inputs(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("inputs")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_inputs(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("inputs", value);
    }
}
impl URLPatternResult {
    pub fn protocol(&self) -> jsbind::Any {
        self.inner.get("protocol").as_::<jsbind::Any>()
    }

    pub fn set_protocol(&mut self, value: jsbind::Any) {
        self.inner.set("protocol", value);
    }
}
impl URLPatternResult {
    pub fn username(&self) -> jsbind::Any {
        self.inner.get("username").as_::<jsbind::Any>()
    }

    pub fn set_username(&mut self, value: jsbind::Any) {
        self.inner.set("username", value);
    }
}
impl URLPatternResult {
    pub fn password(&self) -> jsbind::Any {
        self.inner.get("password").as_::<jsbind::Any>()
    }

    pub fn set_password(&mut self, value: jsbind::Any) {
        self.inner.set("password", value);
    }
}
impl URLPatternResult {
    pub fn hostname(&self) -> jsbind::Any {
        self.inner.get("hostname").as_::<jsbind::Any>()
    }

    pub fn set_hostname(&mut self, value: jsbind::Any) {
        self.inner.set("hostname", value);
    }
}
impl URLPatternResult {
    pub fn port(&self) -> jsbind::Any {
        self.inner.get("port").as_::<jsbind::Any>()
    }

    pub fn set_port(&mut self, value: jsbind::Any) {
        self.inner.set("port", value);
    }
}
impl URLPatternResult {
    pub fn pathname(&self) -> jsbind::Any {
        self.inner.get("pathname").as_::<jsbind::Any>()
    }

    pub fn set_pathname(&mut self, value: jsbind::Any) {
        self.inner.set("pathname", value);
    }
}
impl URLPatternResult {
    pub fn search(&self) -> jsbind::Any {
        self.inner.get("search").as_::<jsbind::Any>()
    }

    pub fn set_search(&mut self, value: jsbind::Any) {
        self.inner.set("search", value);
    }
}
impl URLPatternResult {
    pub fn hash(&self) -> jsbind::Any {
        self.inner.get("hash").as_::<jsbind::Any>()
    }

    pub fn set_hash(&mut self, value: jsbind::Any) {
        self.inner.set("hash", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<URLPattern> for emlite::Val {
    fn from(s: URLPattern) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl URLPattern {
    pub fn new0() -> URLPattern {
        Self {
            inner: emlite::Val::global("URLPattern")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(input: jsbind::Any) -> URLPattern {
        Self {
            inner: emlite::Val::global("URLPattern")
                .new(&[input.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(input: jsbind::Any, options: jsbind::Any) -> URLPattern {
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

    pub fn test1(&self, input: jsbind::Any) -> bool {
        self.inner.call("test", &[input.into()]).as_::<bool>()
    }

    pub fn test2(&self, input: jsbind::Any, base_url: jsbind::USVString) -> bool {
        self.inner
            .call("test", &[input.into(), base_url.into()])
            .as_::<bool>()
    }
}
impl URLPattern {
    pub fn exec0(&self) -> URLPatternResult {
        self.inner.call("exec", &[]).as_::<URLPatternResult>()
    }

    pub fn exec1(&self, input: jsbind::Any) -> URLPatternResult {
        self.inner
            .call("exec", &[input.into()])
            .as_::<URLPatternResult>()
    }

    pub fn exec2(&self, input: jsbind::Any, base_url: jsbind::USVString) -> URLPatternResult {
        self.inner
            .call("exec", &[input.into(), base_url.into()])
            .as_::<URLPatternResult>()
    }
}
impl URLPattern {
    pub fn protocol(&self) -> jsbind::USVString {
        self.inner.get("protocol").as_::<jsbind::USVString>()
    }
}
impl URLPattern {
    pub fn username(&self) -> jsbind::USVString {
        self.inner.get("username").as_::<jsbind::USVString>()
    }
}
impl URLPattern {
    pub fn password(&self) -> jsbind::USVString {
        self.inner.get("password").as_::<jsbind::USVString>()
    }
}
impl URLPattern {
    pub fn hostname(&self) -> jsbind::USVString {
        self.inner.get("hostname").as_::<jsbind::USVString>()
    }
}
impl URLPattern {
    pub fn port(&self) -> jsbind::USVString {
        self.inner.get("port").as_::<jsbind::USVString>()
    }
}
impl URLPattern {
    pub fn pathname(&self) -> jsbind::USVString {
        self.inner.get("pathname").as_::<jsbind::USVString>()
    }
}
impl URLPattern {
    pub fn search(&self) -> jsbind::USVString {
        self.inner.get("search").as_::<jsbind::USVString>()
    }
}
impl URLPattern {
    pub fn hash(&self) -> jsbind::USVString {
        self.inner.get("hash").as_::<jsbind::USVString>()
    }
}
impl URLPattern {
    pub fn has_reg_exp_groups(&self) -> bool {
        self.inner.get("hasRegExpGroups").as_::<bool>()
    }
}
