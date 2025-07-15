use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PannerNode {
    inner: AudioNode,
}
impl FromVal for PannerNode {
    fn from_val(v: &emlite::Val) -> Self {
        PannerNode { inner: AudioNode::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PannerNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PannerNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PannerNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PannerNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PannerNode> for emlite::Val {
    fn from(s: PannerNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PannerNode);



impl PannerNode {
    pub fn new0(context: BaseAudioContext) -> PannerNode {
        Self {
            inner: emlite::Val::global("PannerNode").new(&[context.into()]).as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, options: Any) -> PannerNode {
        Self {
            inner: emlite::Val::global("PannerNode").new(&[context.into(), options.into()]).as_::<AudioNode>(),
        }
    }

}
impl PannerNode {
    pub fn panning_model(&self) -> PanningModelType {
        self.inner.get("panningModel").as_::<PanningModelType>()
    }

    pub fn set_panning_model(&mut self, value: PanningModelType) {
        self.inner.set("panningModel", value);
    }

}
impl PannerNode {
    pub fn position_x(&self) -> AudioParam {
        self.inner.get("positionX").as_::<AudioParam>()
    }

}
impl PannerNode {
    pub fn position_y(&self) -> AudioParam {
        self.inner.get("positionY").as_::<AudioParam>()
    }

}
impl PannerNode {
    pub fn position_z(&self) -> AudioParam {
        self.inner.get("positionZ").as_::<AudioParam>()
    }

}
impl PannerNode {
    pub fn orientation_x(&self) -> AudioParam {
        self.inner.get("orientationX").as_::<AudioParam>()
    }

}
impl PannerNode {
    pub fn orientation_y(&self) -> AudioParam {
        self.inner.get("orientationY").as_::<AudioParam>()
    }

}
impl PannerNode {
    pub fn orientation_z(&self) -> AudioParam {
        self.inner.get("orientationZ").as_::<AudioParam>()
    }

}
impl PannerNode {
    pub fn distance_model(&self) -> DistanceModelType {
        self.inner.get("distanceModel").as_::<DistanceModelType>()
    }

    pub fn set_distance_model(&mut self, value: DistanceModelType) {
        self.inner.set("distanceModel", value);
    }

}
impl PannerNode {
    pub fn ref_distance(&self) -> f64 {
        self.inner.get("refDistance").as_::<f64>()
    }

    pub fn set_ref_distance(&mut self, value: f64) {
        self.inner.set("refDistance", value);
    }

}
impl PannerNode {
    pub fn max_distance(&self) -> f64 {
        self.inner.get("maxDistance").as_::<f64>()
    }

    pub fn set_max_distance(&mut self, value: f64) {
        self.inner.set("maxDistance", value);
    }

}
impl PannerNode {
    pub fn rolloff_factor(&self) -> f64 {
        self.inner.get("rolloffFactor").as_::<f64>()
    }

    pub fn set_rolloff_factor(&mut self, value: f64) {
        self.inner.set("rolloffFactor", value);
    }

}
impl PannerNode {
    pub fn cone_inner_angle(&self) -> f64 {
        self.inner.get("coneInnerAngle").as_::<f64>()
    }

    pub fn set_cone_inner_angle(&mut self, value: f64) {
        self.inner.set("coneInnerAngle", value);
    }

}
impl PannerNode {
    pub fn cone_outer_angle(&self) -> f64 {
        self.inner.get("coneOuterAngle").as_::<f64>()
    }

    pub fn set_cone_outer_angle(&mut self, value: f64) {
        self.inner.set("coneOuterAngle", value);
    }

}
impl PannerNode {
    pub fn cone_outer_gain(&self) -> f64 {
        self.inner.get("coneOuterGain").as_::<f64>()
    }

    pub fn set_cone_outer_gain(&mut self, value: f64) {
        self.inner.set("coneOuterGain", value);
    }

}
impl PannerNode {
    pub fn set_position(&self, x: f32, y: f32, z: f32) -> Undefined {
        self.inner.call("setPosition", &[x.into(), y.into(), z.into(), ]).as_::<Undefined>()
    }

}
impl PannerNode {
    pub fn set_orientation(&self, x: f32, y: f32, z: f32) -> Undefined {
        self.inner.call("setOrientation", &[x.into(), y.into(), z.into(), ]).as_::<Undefined>()
    }

}
