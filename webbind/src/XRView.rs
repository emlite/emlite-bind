use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRView {
    inner: emlite::Val,
}
impl FromVal for XRView {
    fn from_val(v: &emlite::Val) -> Self {
        XRView {
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
impl core::ops::Deref for XRView {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRView {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRView {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRView> for emlite::Val {
    fn from(s: XRView) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRView);

impl XRView {
    pub fn eye(&self) -> XREye {
        self.inner.get("eye").as_::<XREye>()
    }
}
impl XRView {
    pub fn recommended_viewport_scale(&self) -> f64 {
        self.inner.get("recommendedViewportScale").as_::<f64>()
    }
}
impl XRView {
    pub fn request_viewport_scale(&self, scale: f64) -> jsbind::Undefined {
        self.inner
            .call("requestViewportScale", &[scale.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl XRView {
    pub fn camera(&self) -> XRCamera {
        self.inner.get("camera").as_::<XRCamera>()
    }
}
impl XRView {
    pub fn is_first_person_observer(&self) -> bool {
        self.inner.get("isFirstPersonObserver").as_::<bool>()
    }
}
impl XRView {
    pub fn projection_matrix(&self) -> jsbind::Float32Array {
        self.inner
            .get("projectionMatrix")
            .as_::<jsbind::Float32Array>()
    }
}
impl XRView {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }
}
