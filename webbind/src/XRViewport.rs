use super::*;

#[derive(Clone, Debug)]
pub struct XRViewport {
    inner: emlite::Val,
}
impl FromVal for XRViewport {
    fn from_val(v: &emlite::Val) -> Self {
        XRViewport {
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
impl std::ops::Deref for XRViewport {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRViewport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRViewport> for emlite::Val {
    fn from(s: XRViewport) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRViewport {
    pub fn x(&self) -> i32 {
        self.inner.get("x").as_::<i32>()
    }
}
impl XRViewport {
    pub fn y(&self) -> i32 {
        self.inner.get("y").as_::<i32>()
    }
}
impl XRViewport {
    pub fn width(&self) -> i32 {
        self.inner.get("width").as_::<i32>()
    }
}
impl XRViewport {
    pub fn height(&self) -> i32 {
        self.inner.get("height").as_::<i32>()
    }
}
