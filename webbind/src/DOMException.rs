use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DOMException {
    inner: emlite::Val,
}
impl FromVal for DOMException {
    fn from_val(v: &emlite::Val) -> Self {
        DOMException {
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
impl core::ops::Deref for DOMException {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DOMException> for emlite::Val {
    fn from(s: DOMException) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DOMException {
    pub fn new0() -> DOMException {
        Self {
            inner: emlite::Val::global("DOMException")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(message: jsbind::DOMString) -> DOMException {
        Self {
            inner: emlite::Val::global("DOMException")
                .new(&[message.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(message: jsbind::DOMString, name: jsbind::DOMString) -> DOMException {
        Self {
            inner: emlite::Val::global("DOMException")
                .new(&[message.into(), name.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl DOMException {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl DOMException {
    pub fn message(&self) -> jsbind::DOMString {
        self.inner.get("message").as_::<jsbind::DOMString>()
    }
}
impl DOMException {
    pub fn code(&self) -> u16 {
        self.inner.get("code").as_::<u16>()
    }
}
