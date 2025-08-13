use super::*;




/// The PannerOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PannerOptions {
    inner: Any,
}

impl FromVal for PannerOptions {
    fn from_val(v: &Any) -> Self {
        PannerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PannerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PannerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PannerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PannerOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PannerOptions> for Any {
    fn from(s: PannerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PannerOptions> for Any {
    fn from(s: &PannerOptions) -> Any {
        s.inner.clone()
    }
}

impl PannerOptions {
    /// Getter of the `panningModel` attribute.
    pub fn panning_model(&self) -> PanningModelType {
        self.inner.get("panningModel").as_::<PanningModelType>()
    }

    /// Setter of the `panningModel` attribute.
    pub fn set_panning_model(&mut self, value: &PanningModelType) {
        self.inner.set("panningModel", value);
    }
}
impl PannerOptions {
    /// Getter of the `distanceModel` attribute.
    pub fn distance_model(&self) -> DistanceModelType {
        self.inner.get("distanceModel").as_::<DistanceModelType>()
    }

    /// Setter of the `distanceModel` attribute.
    pub fn set_distance_model(&mut self, value: &DistanceModelType) {
        self.inner.set("distanceModel", value);
    }
}
impl PannerOptions {
    /// Getter of the `positionX` attribute.
    pub fn position_x(&self) -> f32 {
        self.inner.get("positionX").as_::<f32>()
    }

    /// Setter of the `positionX` attribute.
    pub fn set_position_x(&mut self, value: f32) {
        self.inner.set("positionX", value);
    }
}
impl PannerOptions {
    /// Getter of the `positionY` attribute.
    pub fn position_y(&self) -> f32 {
        self.inner.get("positionY").as_::<f32>()
    }

    /// Setter of the `positionY` attribute.
    pub fn set_position_y(&mut self, value: f32) {
        self.inner.set("positionY", value);
    }
}
impl PannerOptions {
    /// Getter of the `positionZ` attribute.
    pub fn position_z(&self) -> f32 {
        self.inner.get("positionZ").as_::<f32>()
    }

    /// Setter of the `positionZ` attribute.
    pub fn set_position_z(&mut self, value: f32) {
        self.inner.set("positionZ", value);
    }
}
impl PannerOptions {
    /// Getter of the `orientationX` attribute.
    pub fn orientation_x(&self) -> f32 {
        self.inner.get("orientationX").as_::<f32>()
    }

    /// Setter of the `orientationX` attribute.
    pub fn set_orientation_x(&mut self, value: f32) {
        self.inner.set("orientationX", value);
    }
}
impl PannerOptions {
    /// Getter of the `orientationY` attribute.
    pub fn orientation_y(&self) -> f32 {
        self.inner.get("orientationY").as_::<f32>()
    }

    /// Setter of the `orientationY` attribute.
    pub fn set_orientation_y(&mut self, value: f32) {
        self.inner.set("orientationY", value);
    }
}
impl PannerOptions {
    /// Getter of the `orientationZ` attribute.
    pub fn orientation_z(&self) -> f32 {
        self.inner.get("orientationZ").as_::<f32>()
    }

    /// Setter of the `orientationZ` attribute.
    pub fn set_orientation_z(&mut self, value: f32) {
        self.inner.set("orientationZ", value);
    }
}
impl PannerOptions {
    /// Getter of the `refDistance` attribute.
    pub fn ref_distance(&self) -> f64 {
        self.inner.get("refDistance").as_::<f64>()
    }

    /// Setter of the `refDistance` attribute.
    pub fn set_ref_distance(&mut self, value: f64) {
        self.inner.set("refDistance", value);
    }
}
impl PannerOptions {
    /// Getter of the `maxDistance` attribute.
    pub fn max_distance(&self) -> f64 {
        self.inner.get("maxDistance").as_::<f64>()
    }

    /// Setter of the `maxDistance` attribute.
    pub fn set_max_distance(&mut self, value: f64) {
        self.inner.set("maxDistance", value);
    }
}
impl PannerOptions {
    /// Getter of the `rolloffFactor` attribute.
    pub fn rolloff_factor(&self) -> f64 {
        self.inner.get("rolloffFactor").as_::<f64>()
    }

    /// Setter of the `rolloffFactor` attribute.
    pub fn set_rolloff_factor(&mut self, value: f64) {
        self.inner.set("rolloffFactor", value);
    }
}
impl PannerOptions {
    /// Getter of the `coneInnerAngle` attribute.
    pub fn cone_inner_angle(&self) -> f64 {
        self.inner.get("coneInnerAngle").as_::<f64>()
    }

    /// Setter of the `coneInnerAngle` attribute.
    pub fn set_cone_inner_angle(&mut self, value: f64) {
        self.inner.set("coneInnerAngle", value);
    }
}
impl PannerOptions {
    /// Getter of the `coneOuterAngle` attribute.
    pub fn cone_outer_angle(&self) -> f64 {
        self.inner.get("coneOuterAngle").as_::<f64>()
    }

    /// Setter of the `coneOuterAngle` attribute.
    pub fn set_cone_outer_angle(&mut self, value: f64) {
        self.inner.set("coneOuterAngle", value);
    }
}
impl PannerOptions {
    /// Getter of the `coneOuterGain` attribute.
    pub fn cone_outer_gain(&self) -> f64 {
        self.inner.get("coneOuterGain").as_::<f64>()
    }

    /// Setter of the `coneOuterGain` attribute.
    pub fn set_cone_outer_gain(&mut self, value: f64) {
        self.inner.set("coneOuterGain", value);
    }
}
