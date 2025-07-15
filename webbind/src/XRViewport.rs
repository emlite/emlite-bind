use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for XRViewport {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRViewport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRViewport {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRViewport {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRViewport> for emlite::Val {
    fn from(s: XRViewport) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XRViewport> for emlite::Val {
    fn from(s: &XRViewport) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRViewport);

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
