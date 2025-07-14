use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<URLSearchParams> for emlite::Val {
    fn from(s: URLSearchParams) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl URLSearchParams {
    pub fn new0() -> URLSearchParams {
        Self {
            inner: emlite::Val::global("URLSearchParams")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: jsbind::Any) -> URLSearchParams {
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
    pub fn append(&self, name: jsbind::USVString, value: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("append", &[name.into(), value.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl URLSearchParams {
    pub fn delete0(&self, name: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("delete", &[name.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn delete1(&self, name: jsbind::USVString, value: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("delete", &[name.into(), value.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl URLSearchParams {
    pub fn get(&self, name: jsbind::USVString) -> jsbind::USVString {
        self.inner
            .call("get", &[name.into()])
            .as_::<jsbind::USVString>()
    }
}
impl URLSearchParams {
    pub fn get_all(&self, name: jsbind::USVString) -> jsbind::Sequence<jsbind::USVString> {
        self.inner
            .call("getAll", &[name.into()])
            .as_::<jsbind::Sequence<jsbind::USVString>>()
    }
}
impl URLSearchParams {
    pub fn has0(&self, name: jsbind::USVString) -> bool {
        self.inner.call("has", &[name.into()]).as_::<bool>()
    }

    pub fn has1(&self, name: jsbind::USVString, value: jsbind::USVString) -> bool {
        self.inner
            .call("has", &[name.into(), value.into()])
            .as_::<bool>()
    }
}
impl URLSearchParams {
    pub fn set(&self, name: jsbind::USVString, value: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("set", &[name.into(), value.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl URLSearchParams {
    pub fn sort(&self) -> jsbind::Undefined {
        self.inner.call("sort", &[]).as_::<jsbind::Undefined>()
    }
}
