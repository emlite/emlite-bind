use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRViewerPose {
    inner: XRPose,
}
impl FromVal for XRViewerPose {
    fn from_val(v: &emlite::Val) -> Self {
        XRViewerPose {
            inner: XRPose::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRViewerPose {
    type Target = XRPose;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRViewerPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRViewerPose> for emlite::Val {
    fn from(s: XRViewerPose) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRViewerPose {
    pub fn views(&self) -> jsbind::FrozenArray<XRView> {
        self.inner.get("views").as_::<jsbind::FrozenArray<XRView>>()
    }
}
