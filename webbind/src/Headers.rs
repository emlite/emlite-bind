use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for Headers {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Headers {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Headers {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Headers> for emlite::Val {
    fn from(s: Headers) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Headers);

impl Headers {
    pub fn new0() -> Headers {
        Self {
            inner: emlite::Val::global("Headers").new(&[]).as_::<emlite::Val>(),
        }
    }

    pub fn new1(init: Any) -> Headers {
        Self {
            inner: emlite::Val::global("Headers")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Headers {
    pub fn append(&self, name: ByteString, value: ByteString) -> Undefined {
        self.inner
            .call("append", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
impl Headers {
    pub fn delete(&self, name: ByteString) -> Undefined {
        self.inner.call("delete", &[name.into()]).as_::<Undefined>()
    }
}
impl Headers {
    pub fn get(&self, name: ByteString) -> ByteString {
        self.inner.call("get", &[name.into()]).as_::<ByteString>()
    }
}
impl Headers {
    pub fn get_set_cookie(&self) -> Sequence<ByteString> {
        self.inner
            .call("getSetCookie", &[])
            .as_::<Sequence<ByteString>>()
    }
}
impl Headers {
    pub fn has(&self, name: ByteString) -> bool {
        self.inner.call("has", &[name.into()]).as_::<bool>()
    }
}
impl Headers {
    pub fn set(&self, name: ByteString, value: ByteString) -> Undefined {
        self.inner
            .call("set", &[name.into(), value.into()])
            .as_::<Undefined>()
    }
}
