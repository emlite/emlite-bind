use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRFrame {
    inner: emlite::Val,
}
impl FromVal for XRFrame {
    fn from_val(v: &emlite::Val) -> Self {
        XRFrame {
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
impl core::ops::Deref for XRFrame {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRFrame> for emlite::Val {
    fn from(s: XRFrame) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRFrame {
    pub fn session(&self) -> XRSession {
        self.inner.get("session").as_::<XRSession>()
    }
}
impl XRFrame {
    pub fn predicted_display_time(&self) -> jsbind::Any {
        self.inner.get("predictedDisplayTime").as_::<jsbind::Any>()
    }
}
impl XRFrame {
    pub fn get_viewer_pose(&self, reference_space: XRReferenceSpace) -> XRViewerPose {
        self.inner
            .call("getViewerPose", &[reference_space.into()])
            .as_::<XRViewerPose>()
    }
}
impl XRFrame {
    pub fn get_pose(&self, space: XRSpace, base_space: XRSpace) -> XRPose {
        self.inner
            .call("getPose", &[space.into(), base_space.into()])
            .as_::<XRPose>()
    }
}
impl XRFrame {
    pub fn create_anchor(&self, pose: XRRigidTransform, space: XRSpace) -> jsbind::Promise {
        self.inner
            .call("createAnchor", &[pose.into(), space.into()])
            .as_::<jsbind::Promise>()
    }
}
impl XRFrame {
    pub fn tracked_anchors(&self) -> XRAnchorSet {
        self.inner.get("trackedAnchors").as_::<XRAnchorSet>()
    }
}
impl XRFrame {
    pub fn detected_meshes(&self) -> XRMeshSet {
        self.inner.get("detectedMeshes").as_::<XRMeshSet>()
    }
}
impl XRFrame {
    pub fn get_depth_information(&self, view: XRView) -> XRCPUDepthInformation {
        self.inner
            .call("getDepthInformation", &[view.into()])
            .as_::<XRCPUDepthInformation>()
    }
}
impl XRFrame {
    pub fn get_joint_pose(&self, joint: XRJointSpace, base_space: XRSpace) -> XRJointPose {
        self.inner
            .call("getJointPose", &[joint.into(), base_space.into()])
            .as_::<XRJointPose>()
    }
}
impl XRFrame {
    pub fn fill_joint_radii(
        &self,
        joint_spaces: jsbind::Sequence<XRJointSpace>,
        radii: jsbind::Float32Array,
    ) -> bool {
        self.inner
            .call("fillJointRadii", &[joint_spaces.into(), radii.into()])
            .as_::<bool>()
    }
}
impl XRFrame {
    pub fn fill_poses(
        &self,
        spaces: jsbind::Sequence<XRSpace>,
        base_space: XRSpace,
        transforms: jsbind::Float32Array,
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
    pub fn get_hit_test_results(
        &self,
        hit_test_source: XRHitTestSource,
    ) -> jsbind::Sequence<XRHitTestResult> {
        self.inner
            .call("getHitTestResults", &[hit_test_source.into()])
            .as_::<jsbind::Sequence<XRHitTestResult>>()
    }
}
impl XRFrame {
    pub fn get_hit_test_results_for_transient_input(
        &self,
        hit_test_source: XRTransientInputHitTestSource,
    ) -> jsbind::Sequence<XRTransientInputHitTestResult> {
        self.inner
            .call(
                "getHitTestResultsForTransientInput",
                &[hit_test_source.into()],
            )
            .as_::<jsbind::Sequence<XRTransientInputHitTestResult>>()
    }
}
impl XRFrame {
    pub fn get_light_estimate(&self, light_probe: XRLightProbe) -> XRLightEstimate {
        self.inner
            .call("getLightEstimate", &[light_probe.into()])
            .as_::<XRLightEstimate>()
    }
}
impl XRFrame {
    pub fn detected_planes(&self) -> XRPlaneSet {
        self.inner.get("detectedPlanes").as_::<XRPlaneSet>()
    }
}
