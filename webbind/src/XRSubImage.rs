use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRSubImage {
    inner: emlite::Val,
}
impl FromVal for XRSubImage {
    fn from_val(v: &emlite::Val) -> Self {
        XRSubImage {
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
impl core::ops::Deref for XRSubImage {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRSubImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRSubImage> for emlite::Val {
    fn from(s: XRSubImage) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRSubImage {
    pub fn viewport(&self) -> XRViewport {
        self.inner.get("viewport").as_::<XRViewport>()
    }
}
