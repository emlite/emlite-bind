use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MimeType {
    inner: emlite::Val,
}
impl FromVal for MimeType {
    fn from_val(v: &emlite::Val) -> Self {
        MimeType {
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
impl core::ops::Deref for MimeType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MimeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MimeType> for emlite::Val {
    fn from(s: MimeType) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MimeType {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }
}
impl MimeType {
    pub fn description(&self) -> jsbind::DOMString {
        self.inner.get("description").as_::<jsbind::DOMString>()
    }
}
impl MimeType {
    pub fn suffixes(&self) -> jsbind::DOMString {
        self.inner.get("suffixes").as_::<jsbind::DOMString>()
    }
}
impl MimeType {
    pub fn enabled_plugin(&self) -> Plugin {
        self.inner.get("enabledPlugin").as_::<Plugin>()
    }
}
