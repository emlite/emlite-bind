use super::*;

#[derive(Clone, Debug)]
pub struct Headers {
    inner: emlite::Val,
}
impl FromVal for Headers {
    fn from_val(v: &emlite::Val) -> Self {
        Headers {
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
impl std::ops::Deref for Headers {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Headers> for emlite::Val {
    fn from(s: Headers) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Headers {
    pub fn new0() -> Headers {
        Self {
            inner: emlite::Val::global("Headers").new(&[]).as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: jsbind::Any) -> Headers {
        Self {
            inner: emlite::Val::global("Headers")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Headers {
    pub fn append(&self, name: jsbind::ByteString, value: jsbind::ByteString) -> jsbind::Undefined {
        self.inner
            .call("append", &[name.into(), value.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Headers {
    pub fn delete(&self, name: jsbind::ByteString) -> jsbind::Undefined {
        self.inner
            .call("delete", &[name.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Headers {
    pub fn get(&self, name: jsbind::ByteString) -> jsbind::ByteString {
        self.inner
            .call("get", &[name.into()])
            .as_::<jsbind::ByteString>()
    }
}
impl Headers {
    pub fn get_set_cookie(&self) -> jsbind::Sequence<jsbind::ByteString> {
        self.inner
            .call("getSetCookie", &[])
            .as_::<jsbind::Sequence<jsbind::ByteString>>()
    }
}
impl Headers {
    pub fn has(&self, name: jsbind::ByteString) -> bool {
        self.inner.call("has", &[name.into()]).as_::<bool>()
    }
}
impl Headers {
    pub fn set(&self, name: jsbind::ByteString, value: jsbind::ByteString) -> jsbind::Undefined {
        self.inner
            .call("set", &[name.into(), value.into()])
            .as_::<jsbind::Undefined>()
    }
}
