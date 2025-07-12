use super::*;

#[derive(Clone, Debug)]
pub struct XRCamera {
    inner: emlite::Val,
}
impl FromVal for XRCamera {
    fn from_val(v: &emlite::Val) -> Self {
        XRCamera {
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
impl std::ops::Deref for XRCamera {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRCamera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRCamera> for emlite::Val {
    fn from(s: XRCamera) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRCamera {
    pub fn width(&self) -> u32 {
        self.inner.get("width").as_::<u32>()
    }
}
impl XRCamera {
    pub fn height(&self) -> u32 {
        self.inner.get("height").as_::<u32>()
    }
}
