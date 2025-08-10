use super::*;

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
    pub fn panning_model(&self) -> PanningModelType {
        self.inner.get("panningModel").as_::<PanningModelType>()
    }

    pub fn set_panning_model(&mut self, value: &PanningModelType) {
        self.inner.set("panningModel", value);
    }
}
impl PannerOptions {
    pub fn distance_model(&self) -> DistanceModelType {
        self.inner.get("distanceModel").as_::<DistanceModelType>()
    }

    pub fn set_distance_model(&mut self, value: &DistanceModelType) {
        self.inner.set("distanceModel", value);
    }
}
impl PannerOptions {
    pub fn position_x(&self) -> f32 {
        self.inner.get("positionX").as_::<f32>()
    }

    pub fn set_position_x(&mut self, value: f32) {
        self.inner.set("positionX", value);
    }
}
impl PannerOptions {
    pub fn position_y(&self) -> f32 {
        self.inner.get("positionY").as_::<f32>()
    }

    pub fn set_position_y(&mut self, value: f32) {
        self.inner.set("positionY", value);
    }
}
impl PannerOptions {
    pub fn position_z(&self) -> f32 {
        self.inner.get("positionZ").as_::<f32>()
    }

    pub fn set_position_z(&mut self, value: f32) {
        self.inner.set("positionZ", value);
    }
}
impl PannerOptions {
    pub fn orientation_x(&self) -> f32 {
        self.inner.get("orientationX").as_::<f32>()
    }

    pub fn set_orientation_x(&mut self, value: f32) {
        self.inner.set("orientationX", value);
    }
}
impl PannerOptions {
    pub fn orientation_y(&self) -> f32 {
        self.inner.get("orientationY").as_::<f32>()
    }

    pub fn set_orientation_y(&mut self, value: f32) {
        self.inner.set("orientationY", value);
    }
}
impl PannerOptions {
    pub fn orientation_z(&self) -> f32 {
        self.inner.get("orientationZ").as_::<f32>()
    }

    pub fn set_orientation_z(&mut self, value: f32) {
        self.inner.set("orientationZ", value);
    }
}
impl PannerOptions {
    pub fn ref_distance(&self) -> f64 {
        self.inner.get("refDistance").as_::<f64>()
    }

    pub fn set_ref_distance(&mut self, value: f64) {
        self.inner.set("refDistance", value);
    }
}
impl PannerOptions {
    pub fn max_distance(&self) -> f64 {
        self.inner.get("maxDistance").as_::<f64>()
    }

    pub fn set_max_distance(&mut self, value: f64) {
        self.inner.set("maxDistance", value);
    }
}
impl PannerOptions {
    pub fn rolloff_factor(&self) -> f64 {
        self.inner.get("rolloffFactor").as_::<f64>()
    }

    pub fn set_rolloff_factor(&mut self, value: f64) {
        self.inner.set("rolloffFactor", value);
    }
}
impl PannerOptions {
    pub fn cone_inner_angle(&self) -> f64 {
        self.inner.get("coneInnerAngle").as_::<f64>()
    }

    pub fn set_cone_inner_angle(&mut self, value: f64) {
        self.inner.set("coneInnerAngle", value);
    }
}
impl PannerOptions {
    pub fn cone_outer_angle(&self) -> f64 {
        self.inner.get("coneOuterAngle").as_::<f64>()
    }

    pub fn set_cone_outer_angle(&mut self, value: f64) {
        self.inner.set("coneOuterAngle", value);
    }
}
impl PannerOptions {
    pub fn cone_outer_gain(&self) -> f64 {
        self.inner.get("coneOuterGain").as_::<f64>()
    }

    pub fn set_cone_outer_gain(&mut self, value: f64) {
        self.inner.set("coneOuterGain", value);
    }
}
