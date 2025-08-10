use super::*;

/// The PannerNode class.
/// [`PannerNode`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PannerNode {
    inner: AudioNode,
}
impl FromVal for PannerNode {
    fn from_val(v: &Any) -> Self {
        PannerNode {
            inner: AudioNode::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for PannerNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PannerNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PannerNode> for Any {
    fn from(s: PannerNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PannerNode> for Any {
    fn from(s: &PannerNode) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PannerNode);

impl PannerNode {
    /// The `new PannerNode(..)` constructor, creating a new PannerNode instance
    pub fn new0(context: &BaseAudioContext) -> PannerNode {
        Self {
            inner: Any::global("PannerNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    /// The `new PannerNode(..)` constructor, creating a new PannerNode instance
    pub fn new1(context: &BaseAudioContext, options: &PannerOptions) -> PannerNode {
        Self {
            inner: Any::global("PannerNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl PannerNode {
    /// Getter of the `panningModel` attribute.
    /// [`PannerNode.panningModel`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)
    pub fn panning_model(&self) -> PanningModelType {
        self.inner.get("panningModel").as_::<PanningModelType>()
    }

    /// Setter of the `panningModel` attribute.
    /// [`PannerNode.panningModel`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/panningModel)
    pub fn set_panning_model(&mut self, value: &PanningModelType) {
        self.inner.set("panningModel", value);
    }
}
impl PannerNode {
    /// Getter of the `positionX` attribute.
    /// [`PannerNode.positionX`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionX)
    pub fn position_x(&self) -> AudioParam {
        self.inner.get("positionX").as_::<AudioParam>()
    }
}
impl PannerNode {
    /// Getter of the `positionY` attribute.
    /// [`PannerNode.positionY`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionY)
    pub fn position_y(&self) -> AudioParam {
        self.inner.get("positionY").as_::<AudioParam>()
    }
}
impl PannerNode {
    /// Getter of the `positionZ` attribute.
    /// [`PannerNode.positionZ`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/positionZ)
    pub fn position_z(&self) -> AudioParam {
        self.inner.get("positionZ").as_::<AudioParam>()
    }
}
impl PannerNode {
    /// Getter of the `orientationX` attribute.
    /// [`PannerNode.orientationX`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationX)
    pub fn orientation_x(&self) -> AudioParam {
        self.inner.get("orientationX").as_::<AudioParam>()
    }
}
impl PannerNode {
    /// Getter of the `orientationY` attribute.
    /// [`PannerNode.orientationY`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationY)
    pub fn orientation_y(&self) -> AudioParam {
        self.inner.get("orientationY").as_::<AudioParam>()
    }
}
impl PannerNode {
    /// Getter of the `orientationZ` attribute.
    /// [`PannerNode.orientationZ`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/orientationZ)
    pub fn orientation_z(&self) -> AudioParam {
        self.inner.get("orientationZ").as_::<AudioParam>()
    }
}
impl PannerNode {
    /// Getter of the `distanceModel` attribute.
    /// [`PannerNode.distanceModel`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)
    pub fn distance_model(&self) -> DistanceModelType {
        self.inner.get("distanceModel").as_::<DistanceModelType>()
    }

    /// Setter of the `distanceModel` attribute.
    /// [`PannerNode.distanceModel`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/distanceModel)
    pub fn set_distance_model(&mut self, value: &DistanceModelType) {
        self.inner.set("distanceModel", value);
    }
}
impl PannerNode {
    /// Getter of the `refDistance` attribute.
    /// [`PannerNode.refDistance`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)
    pub fn ref_distance(&self) -> f64 {
        self.inner.get("refDistance").as_::<f64>()
    }

    /// Setter of the `refDistance` attribute.
    /// [`PannerNode.refDistance`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/refDistance)
    pub fn set_ref_distance(&mut self, value: f64) {
        self.inner.set("refDistance", value);
    }
}
impl PannerNode {
    /// Getter of the `maxDistance` attribute.
    /// [`PannerNode.maxDistance`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)
    pub fn max_distance(&self) -> f64 {
        self.inner.get("maxDistance").as_::<f64>()
    }

    /// Setter of the `maxDistance` attribute.
    /// [`PannerNode.maxDistance`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/maxDistance)
    pub fn set_max_distance(&mut self, value: f64) {
        self.inner.set("maxDistance", value);
    }
}
impl PannerNode {
    /// Getter of the `rolloffFactor` attribute.
    /// [`PannerNode.rolloffFactor`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)
    pub fn rolloff_factor(&self) -> f64 {
        self.inner.get("rolloffFactor").as_::<f64>()
    }

    /// Setter of the `rolloffFactor` attribute.
    /// [`PannerNode.rolloffFactor`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/rolloffFactor)
    pub fn set_rolloff_factor(&mut self, value: f64) {
        self.inner.set("rolloffFactor", value);
    }
}
impl PannerNode {
    /// Getter of the `coneInnerAngle` attribute.
    /// [`PannerNode.coneInnerAngle`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)
    pub fn cone_inner_angle(&self) -> f64 {
        self.inner.get("coneInnerAngle").as_::<f64>()
    }

    /// Setter of the `coneInnerAngle` attribute.
    /// [`PannerNode.coneInnerAngle`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneInnerAngle)
    pub fn set_cone_inner_angle(&mut self, value: f64) {
        self.inner.set("coneInnerAngle", value);
    }
}
impl PannerNode {
    /// Getter of the `coneOuterAngle` attribute.
    /// [`PannerNode.coneOuterAngle`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)
    pub fn cone_outer_angle(&self) -> f64 {
        self.inner.get("coneOuterAngle").as_::<f64>()
    }

    /// Setter of the `coneOuterAngle` attribute.
    /// [`PannerNode.coneOuterAngle`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterAngle)
    pub fn set_cone_outer_angle(&mut self, value: f64) {
        self.inner.set("coneOuterAngle", value);
    }
}
impl PannerNode {
    /// Getter of the `coneOuterGain` attribute.
    /// [`PannerNode.coneOuterGain`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)
    pub fn cone_outer_gain(&self) -> f64 {
        self.inner.get("coneOuterGain").as_::<f64>()
    }

    /// Setter of the `coneOuterGain` attribute.
    /// [`PannerNode.coneOuterGain`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/coneOuterGain)
    pub fn set_cone_outer_gain(&mut self, value: f64) {
        self.inner.set("coneOuterGain", value);
    }
}
impl PannerNode {
    /// The setPosition method.
    /// [`PannerNode.setPosition`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setPosition)
    pub fn set_position(&self, x: f32, y: f32, z: f32) -> Undefined {
        self.inner
            .call("setPosition", &[x.into(), y.into(), z.into()])
            .as_::<Undefined>()
    }
}
impl PannerNode {
    /// The setOrientation method.
    /// [`PannerNode.setOrientation`](https://developer.mozilla.org/en-US/docs/Web/API/PannerNode/setOrientation)
    pub fn set_orientation(&self, x: f32, y: f32, z: f32) -> Undefined {
        self.inner
            .call("setOrientation", &[x.into(), y.into(), z.into()])
            .as_::<Undefined>()
    }
}
