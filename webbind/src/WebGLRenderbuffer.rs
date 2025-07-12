use super::*;

#[derive(Clone, Debug)]
pub struct WebGLRenderbuffer {
    inner: WebGLObject,
}
impl FromVal for WebGLRenderbuffer {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLRenderbuffer {
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
impl std::ops::Deref for WebGLRenderbuffer {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebGLRenderbuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebGLRenderbuffer> for emlite::Val {
    fn from(s: WebGLRenderbuffer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
