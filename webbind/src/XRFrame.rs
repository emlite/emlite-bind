use super::*;

/// The XRFrame class.
/// [`XRFrame`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRFrame {
    inner: Any,
}
impl FromVal for XRFrame {
    fn from_val(v: &Any) -> Self {
        XRFrame {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRFrame {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRFrame {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRFrame {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRFrame> for Any {
    fn from(s: XRFrame) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRFrame> for Any {
    fn from(s: &XRFrame) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRFrame);

impl XRFrame {
    /// Getter of the `session` attribute.
    /// [`XRFrame.session`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/session)
    pub fn session(&self) -> XRSession {
        self.inner.get("session").as_::<XRSession>()
    }
}
impl XRFrame {
    /// Getter of the `predictedDisplayTime` attribute.
    /// [`XRFrame.predictedDisplayTime`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/predictedDisplayTime)
    pub fn predicted_display_time(&self) -> Any {
        self.inner.get("predictedDisplayTime").as_::<Any>()
    }
}
impl XRFrame {
    /// The getViewerPose method.
    /// [`XRFrame.getViewerPose`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getViewerPose)
    pub fn get_viewer_pose(&self, reference_space: &XRReferenceSpace) -> XRViewerPose {
        self.inner
            .call("getViewerPose", &[reference_space.into()])
            .as_::<XRViewerPose>()
    }
}
impl XRFrame {
    /// The getPose method.
    /// [`XRFrame.getPose`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getPose)
    pub fn get_pose(&self, space: &XRSpace, base_space: &XRSpace) -> XRPose {
        self.inner
            .call("getPose", &[space.into(), base_space.into()])
            .as_::<XRPose>()
    }
}
impl XRFrame {
    /// The createAnchor method.
    /// [`XRFrame.createAnchor`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/createAnchor)
    pub fn create_anchor(&self, pose: &XRRigidTransform, space: &XRSpace) -> Promise {
        self.inner
            .call("createAnchor", &[pose.into(), space.into()])
            .as_::<Promise>()
    }
}
impl XRFrame {
    /// Getter of the `trackedAnchors` attribute.
    /// [`XRFrame.trackedAnchors`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/trackedAnchors)
    pub fn tracked_anchors(&self) -> XRAnchorSet {
        self.inner.get("trackedAnchors").as_::<XRAnchorSet>()
    }
}
impl XRFrame {
    /// Getter of the `detectedMeshes` attribute.
    /// [`XRFrame.detectedMeshes`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/detectedMeshes)
    pub fn detected_meshes(&self) -> XRMeshSet {
        self.inner.get("detectedMeshes").as_::<XRMeshSet>()
    }
}
impl XRFrame {
    /// The getDepthInformation method.
    /// [`XRFrame.getDepthInformation`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getDepthInformation)
    pub fn get_depth_information(&self, view: &XRView) -> XRCPUDepthInformation {
        self.inner
            .call("getDepthInformation", &[view.into()])
            .as_::<XRCPUDepthInformation>()
    }
}
impl XRFrame {
    /// The getJointPose method.
    /// [`XRFrame.getJointPose`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getJointPose)
    pub fn get_joint_pose(&self, joint: &XRJointSpace, base_space: &XRSpace) -> XRJointPose {
        self.inner
            .call("getJointPose", &[joint.into(), base_space.into()])
            .as_::<XRJointPose>()
    }
}
impl XRFrame {
    /// The fillJointRadii method.
    /// [`XRFrame.fillJointRadii`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/fillJointRadii)
    pub fn fill_joint_radii(
        &self,
        joint_spaces: &Sequence<XRJointSpace>,
        radii: &Float32Array,
    ) -> bool {
        self.inner
            .call("fillJointRadii", &[joint_spaces.into(), radii.into()])
            .as_::<bool>()
    }
}
impl XRFrame {
    /// The fillPoses method.
    /// [`XRFrame.fillPoses`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/fillPoses)
    pub fn fill_poses(
        &self,
        spaces: &Sequence<XRSpace>,
        base_space: &XRSpace,
        transforms: &Float32Array,
    ) -> bool {
        self.inner
            .call(
                "fillPoses",
                &[spaces.into(), base_space.into(), transforms.into()],
            )
            .as_::<bool>()
    }
}
impl XRFrame {
    /// The getHitTestResults method.
    /// [`XRFrame.getHitTestResults`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getHitTestResults)
    pub fn get_hit_test_results(
        &self,
        hit_test_source: &XRHitTestSource,
    ) -> Sequence<XRHitTestResult> {
        self.inner
            .call("getHitTestResults", &[hit_test_source.into()])
            .as_::<Sequence<XRHitTestResult>>()
    }
}
impl XRFrame {
    /// The getHitTestResultsForTransientInput method.
    /// [`XRFrame.getHitTestResultsForTransientInput`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getHitTestResultsForTransientInput)
    pub fn get_hit_test_results_for_transient_input(
        &self,
        hit_test_source: &XRTransientInputHitTestSource,
    ) -> Sequence<XRTransientInputHitTestResult> {
        self.inner
            .call(
                "getHitTestResultsForTransientInput",
                &[hit_test_source.into()],
            )
            .as_::<Sequence<XRTransientInputHitTestResult>>()
    }
}
impl XRFrame {
    /// The getLightEstimate method.
    /// [`XRFrame.getLightEstimate`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getLightEstimate)
    pub fn get_light_estimate(&self, light_probe: &XRLightProbe) -> XRLightEstimate {
        self.inner
            .call("getLightEstimate", &[light_probe.into()])
            .as_::<XRLightEstimate>()
    }
}
impl XRFrame {
    /// Getter of the `detectedPlanes` attribute.
    /// [`XRFrame.detectedPlanes`](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/detectedPlanes)
    pub fn detected_planes(&self) -> XRPlaneSet {
        self.inner.get("detectedPlanes").as_::<XRPlaneSet>()
    }
}
