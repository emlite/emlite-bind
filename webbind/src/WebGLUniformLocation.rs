use super::*;

#[derive(Clone, Debug)]
pub struct WebGLUniformLocation {
    inner: emlite::Val,
}
impl FromVal for WebGLUniformLocation {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLUniformLocation {
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
impl std::ops::Deref for WebGLUniformLocation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebGLUniformLocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebGLUniformLocation> for emlite::Val {
    fn from(s: WebGLUniformLocation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
