use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for XRBoundedReferenceSpace {
    type Target = XRReferenceSpace;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRBoundedReferenceSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRBoundedReferenceSpace {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRBoundedReferenceSpace {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRBoundedReferenceSpace> for emlite::Val {
    fn from(s: XRBoundedReferenceSpace) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRBoundedReferenceSpace);

impl XRBoundedReferenceSpace {
    pub fn bounds_geometry(&self) -> jsbind::FrozenArray<DOMPointReadOnly> {
        self.inner
            .get("boundsGeometry")
            .as_::<jsbind::FrozenArray<DOMPointReadOnly>>()
    }
}
