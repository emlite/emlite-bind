use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WebGLQuery {
    inner: WebGLObject,
}
impl FromVal for WebGLQuery {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLQuery {
            inner: WebGLObject::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WebGLQuery {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLQuery {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebGLQuery> for emlite::Val {
    fn from(s: WebGLQuery) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
