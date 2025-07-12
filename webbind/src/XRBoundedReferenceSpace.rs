use super::*;

#[derive(Clone, Debug)]
pub struct XRBoundedReferenceSpace {
    inner: XRReferenceSpace,
}
impl FromVal for XRBoundedReferenceSpace {
    fn from_val(v: &emlite::Val) -> Self {
        XRBoundedReferenceSpace {
            inner: XRReferenceSpace::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for XRBoundedReferenceSpace {
    type Target = XRReferenceSpace;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRBoundedReferenceSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRBoundedReferenceSpace> for emlite::Val {
    fn from(s: XRBoundedReferenceSpace) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRBoundedReferenceSpace {
    pub fn bounds_geometry(&self) -> jsbind::FrozenArray<DOMPointReadOnly> {
        self.inner
            .get("boundsGeometry")
            .as_::<jsbind::FrozenArray<DOMPointReadOnly>>()
    }
}
