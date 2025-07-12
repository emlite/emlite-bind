use super::*;

#[derive(Clone, Debug)]
pub struct WebGLShader {
    inner: WebGLObject,
}
impl FromVal for WebGLShader {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLShader {
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
impl std::ops::Deref for WebGLShader {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebGLShader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebGLShader> for emlite::Val {
    fn from(s: WebGLShader) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
