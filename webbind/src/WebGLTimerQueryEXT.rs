use super::*;

#[derive(Clone, Debug)]
pub struct WebGLTimerQueryEXT {
    inner: WebGLObject,
}
impl FromVal for WebGLTimerQueryEXT {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLTimerQueryEXT {
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
impl std::ops::Deref for WebGLTimerQueryEXT {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebGLTimerQueryEXT {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebGLTimerQueryEXT> for emlite::Val {
    fn from(s: WebGLTimerQueryEXT) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
