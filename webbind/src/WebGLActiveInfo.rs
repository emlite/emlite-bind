use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WebGLActiveInfo {
    inner: emlite::Val,
}
impl FromVal for WebGLActiveInfo {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLActiveInfo {
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
impl core::ops::Deref for WebGLActiveInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLActiveInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebGLActiveInfo> for emlite::Val {
    fn from(s: WebGLActiveInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebGLActiveInfo {
    pub fn size(&self) -> jsbind::Any {
        self.inner.get("size").as_::<jsbind::Any>()
    }
}
impl WebGLActiveInfo {
    pub fn type_(&self) -> jsbind::Any {
        self.inner.get("type").as_::<jsbind::Any>()
    }
}
impl WebGLActiveInfo {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
