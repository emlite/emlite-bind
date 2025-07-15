use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLSearchParams {
    inner: emlite::Val,
}
impl FromVal for URLSearchParams {
    fn from_val(v: &emlite::Val) -> Self {
        URLSearchParams {
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
impl core::ops::Deref for URLSearchParams {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for URLSearchParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for URLSearchParams {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for URLSearchParams {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<URLSearchParams> for emlite::Val {
    fn from(s: URLSearchParams) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(URLSearchParams);

impl URLSearchParams {
    pub fn new0() -> URLSearchParams {
        Self {
            inner: emlite::Val::global("URLSearchParams")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: Any) -> URLSearchParams {
        Self {
            inner: emlite::Val::global("URLSearchParams")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl URLSearchParams {
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }
}
impl URLSearchParams {
    pub fn append(&self, name: USVString, value: USVString) -> Undefined {
        self.inner
            .call("append", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl URLSearchParams {
    pub fn delete0(&self, name: USVString) -> Undefined {
        self.inner.call("delete", &[name.into()]).as_::<Undefined>()
    }

    pub fn delete1(&self, name: USVString, value: USVString) -> Undefined {
        self.inner
            .call("delete", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl URLSearchParams {
    pub fn get(&self, name: USVString) -> USVString {
        self.inner.call("get", &[name.into()]).as_::<USVString>()
    }
}
impl URLSearchParams {
    pub fn get_all(&self, name: USVString) -> Sequence<USVString> {
        self.inner
            .call("getAll", &[name.into()])
            .as_::<Sequence<USVString>>()
    }
}
impl URLSearchParams {
    pub fn has0(&self, name: USVString) -> bool {
        self.inner.call("has", &[name.into()]).as_::<bool>()
    }

    pub fn has1(&self, name: USVString, value: USVString) -> bool {
        self.inner
            .call("has", &[name.into(), value.into()])
            .as_::<bool>()
    }
}
impl URLSearchParams {
    pub fn set(&self, name: USVString, value: USVString) -> Undefined {
        self.inner
            .call("set", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl URLSearchParams {
    pub fn sort(&self) -> Undefined {
        self.inner.call("sort", &[]).as_::<Undefined>()
    }
}
