use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WebGLObject {
    inner: emlite::Val,
}
impl FromVal for WebGLObject {
    fn from_val(v: &emlite::Val) -> Self {
        WebGLObject {
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
impl core::ops::Deref for WebGLObject {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebGLObject> for emlite::Val {
    fn from(s: WebGLObject) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebGLObject {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
