use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRLightEstimate {
    inner: emlite::Val,
}
impl FromVal for XRLightEstimate {
    fn from_val(v: &emlite::Val) -> Self {
        XRLightEstimate {
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
impl core::ops::Deref for XRLightEstimate {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRLightEstimate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRLightEstimate> for emlite::Val {
    fn from(s: XRLightEstimate) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRLightEstimate {
    pub fn spherical_harmonics_coefficients(&self) -> jsbind::Float32Array {
        self.inner
            .get("sphericalHarmonicsCoefficients")
            .as_::<jsbind::Float32Array>()
    }
}
impl XRLightEstimate {
    pub fn primary_light_direction(&self) -> DOMPointReadOnly {
        self.inner
            .get("primaryLightDirection")
            .as_::<DOMPointReadOnly>()
    }
}
impl XRLightEstimate {
    pub fn primary_light_intensity(&self) -> DOMPointReadOnly {
        self.inner
            .get("primaryLightIntensity")
            .as_::<DOMPointReadOnly>()
    }
}
