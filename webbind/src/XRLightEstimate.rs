use super::*;




/// The XRLightEstimate class.
/// [`XRLightEstimate`](https://developer.mozilla.org/en-US/docs/Web/API/XRLightEstimate)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLightEstimate {
    inner: Any,
}

impl FromVal for XRLightEstimate {
    fn from_val(v: &Any) -> Self {
        XRLightEstimate { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRLightEstimate {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRLightEstimate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRLightEstimate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRLightEstimate {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRLightEstimate> for Any {
    fn from(s: XRLightEstimate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRLightEstimate> for Any {
    fn from(s: &XRLightEstimate) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRLightEstimate);


impl XRLightEstimate {
    /// Getter of the `sphericalHarmonicsCoefficients` attribute.
    /// [`XRLightEstimate.sphericalHarmonicsCoefficients`](https://developer.mozilla.org/en-US/docs/Web/API/XRLightEstimate/sphericalHarmonicsCoefficients)
    pub fn spherical_harmonics_coefficients(&self) -> Float32Array {
        self.inner.get("sphericalHarmonicsCoefficients").as_::<Float32Array>()
    }

}
impl XRLightEstimate {
    /// Getter of the `primaryLightDirection` attribute.
    /// [`XRLightEstimate.primaryLightDirection`](https://developer.mozilla.org/en-US/docs/Web/API/XRLightEstimate/primaryLightDirection)
    pub fn primary_light_direction(&self) -> DOMPointReadOnly {
        self.inner.get("primaryLightDirection").as_::<DOMPointReadOnly>()
    }

}
impl XRLightEstimate {
    /// Getter of the `primaryLightIntensity` attribute.
    /// [`XRLightEstimate.primaryLightIntensity`](https://developer.mozilla.org/en-US/docs/Web/API/XRLightEstimate/primaryLightIntensity)
    pub fn primary_light_intensity(&self) -> DOMPointReadOnly {
        self.inner.get("primaryLightIntensity").as_::<DOMPointReadOnly>()
    }

}
