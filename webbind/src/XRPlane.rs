use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRPlane {
    inner: emlite::Val,
}
impl FromVal for XRPlane {
    fn from_val(v: &emlite::Val) -> Self {
        XRPlane {
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
impl core::ops::Deref for XRPlane {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRPlane {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRPlane {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRPlane {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRPlane> for emlite::Val {
    fn from(s: XRPlane) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRPlane);

impl XRPlane {
    pub fn plane_space(&self) -> XRSpace {
        self.inner.get("planeSpace").as_::<XRSpace>()
    }
}
impl XRPlane {
    pub fn polygon(&self) -> jsbind::FrozenArray<DOMPointReadOnly> {
        self.inner
            .get("polygon")
            .as_::<jsbind::FrozenArray<DOMPointReadOnly>>()
    }
}
impl XRPlane {
    pub fn orientation(&self) -> XRPlaneOrientation {
        self.inner.get("orientation").as_::<XRPlaneOrientation>()
    }
}
impl XRPlane {
    pub fn last_changed_time(&self) -> jsbind::Any {
        self.inner.get("lastChangedTime").as_::<jsbind::Any>()
    }
}
impl XRPlane {
    pub fn semantic_label(&self) -> jsbind::DOMString {
        self.inner.get("semanticLabel").as_::<jsbind::DOMString>()
    }
}
