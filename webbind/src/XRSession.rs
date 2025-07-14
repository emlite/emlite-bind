use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRRenderStateInit {
    inner: emlite::Val,
}
impl FromVal for XRRenderStateInit {
    fn from_val(v: &emlite::Val) -> Self {
        XRRenderStateInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRRenderStateInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRRenderStateInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRRenderStateInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRRenderStateInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRRenderStateInit> for emlite::Val {
    fn from(s: XRRenderStateInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRRenderStateInit {
    pub fn depth_near(&self) -> f64 {
        self.inner.get("depthNear").as_::<f64>()
    }

    pub fn set_depth_near(&mut self, value: f64) {
        self.inner.set("depthNear", value);
    }
}
impl XRRenderStateInit {
    pub fn depth_far(&self) -> f64 {
        self.inner.get("depthFar").as_::<f64>()
    }

    pub fn set_depth_far(&mut self, value: f64) {
        self.inner.set("depthFar", value);
    }
}
impl XRRenderStateInit {
    pub fn passthrough_fully_obscured(&self) -> bool {
        self.inner.get("passthroughFullyObscured").as_::<bool>()
    }

    pub fn set_passthrough_fully_obscured(&mut self, value: bool) {
        self.inner.set("passthroughFullyObscured", value);
    }
}
impl XRRenderStateInit {
    pub fn inline_vertical_field_of_view(&self) -> f64 {
        self.inner.get("inlineVerticalFieldOfView").as_::<f64>()
    }

    pub fn set_inline_vertical_field_of_view(&mut self, value: f64) {
        self.inner.set("inlineVerticalFieldOfView", value);
    }
}
impl XRRenderStateInit {
    pub fn base_layer(&self) -> XRWebGLLayer {
        self.inner.get("baseLayer").as_::<XRWebGLLayer>()
    }

    pub fn set_base_layer(&mut self, value: XRWebGLLayer) {
        self.inner.set("baseLayer", value);
    }
}
impl XRRenderStateInit {
    pub fn layers(&self) -> jsbind::Sequence<XRLayer> {
        self.inner.get("layers").as_::<jsbind::Sequence<XRLayer>>()
    }

    pub fn set_layers(&mut self, value: jsbind::Sequence<XRLayer>) {
        self.inner.set("layers", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRDOMOverlayState {
    inner: emlite::Val,
}
impl FromVal for XRDOMOverlayState {
    fn from_val(v: &emlite::Val) -> Self {
        XRDOMOverlayState { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRDOMOverlayState {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRDOMOverlayState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRDOMOverlayState {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRDOMOverlayState {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRDOMOverlayState> for emlite::Val {
    fn from(s: XRDOMOverlayState) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRDOMOverlayState {
    pub fn type_(&self) -> XRDOMOverlayType {
        self.inner.get("type").as_::<XRDOMOverlayType>()
    }

    pub fn set_type_(&mut self, value: XRDOMOverlayType) {
        self.inner.set("type", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRHitTestOptionsInit {
    inner: emlite::Val,
}
impl FromVal for XRHitTestOptionsInit {
    fn from_val(v: &emlite::Val) -> Self {
        XRHitTestOptionsInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRHitTestOptionsInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRHitTestOptionsInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRHitTestOptionsInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRHitTestOptionsInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRHitTestOptionsInit> for emlite::Val {
    fn from(s: XRHitTestOptionsInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRHitTestOptionsInit {
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    pub fn set_space(&mut self, value: XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRHitTestOptionsInit {
    pub fn entity_types(&self) -> jsbind::Sequence<XRHitTestTrackableType> {
        self.inner
            .get("entityTypes")
            .as_::<jsbind::Sequence<XRHitTestTrackableType>>()
    }

    pub fn set_entity_types(&mut self, value: jsbind::Sequence<XRHitTestTrackableType>) {
        self.inner.set("entityTypes", value);
    }
}
impl XRHitTestOptionsInit {
    pub fn offset_ray(&self) -> XRRay {
        self.inner.get("offsetRay").as_::<XRRay>()
    }

    pub fn set_offset_ray(&mut self, value: XRRay) {
        self.inner.set("offsetRay", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRTransientInputHitTestOptionsInit {
    inner: emlite::Val,
}
impl FromVal for XRTransientInputHitTestOptionsInit {
    fn from_val(v: &emlite::Val) -> Self {
        XRTransientInputHitTestOptionsInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRTransientInputHitTestOptionsInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRTransientInputHitTestOptionsInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRTransientInputHitTestOptionsInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRTransientInputHitTestOptionsInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRTransientInputHitTestOptionsInit> for emlite::Val {
    fn from(s: XRTransientInputHitTestOptionsInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRTransientInputHitTestOptionsInit {
    pub fn profile(&self) -> jsbind::DOMString {
        self.inner.get("profile").as_::<jsbind::DOMString>()
    }

    pub fn set_profile(&mut self, value: jsbind::DOMString) {
        self.inner.set("profile", value);
    }
}
impl XRTransientInputHitTestOptionsInit {
    pub fn entity_types(&self) -> jsbind::Sequence<XRHitTestTrackableType> {
        self.inner
            .get("entityTypes")
            .as_::<jsbind::Sequence<XRHitTestTrackableType>>()
    }

    pub fn set_entity_types(&mut self, value: jsbind::Sequence<XRHitTestTrackableType>) {
        self.inner.set("entityTypes", value);
    }
}
impl XRTransientInputHitTestOptionsInit {
    pub fn offset_ray(&self) -> XRRay {
        self.inner.get("offsetRay").as_::<XRRay>()
    }

    pub fn set_offset_ray(&mut self, value: XRRay) {
        self.inner.set("offsetRay", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLightProbeInit {
    inner: emlite::Val,
}
impl FromVal for XRLightProbeInit {
    fn from_val(v: &emlite::Val) -> Self {
        XRLightProbeInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRLightProbeInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRLightProbeInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRLightProbeInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRLightProbeInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRLightProbeInit> for emlite::Val {
    fn from(s: XRLightProbeInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRLightProbeInit {
    pub fn reflection_format(&self) -> XRReflectionFormat {
        self.inner
            .get("reflectionFormat")
            .as_::<XRReflectionFormat>()
    }

    pub fn set_reflection_format(&mut self, value: XRReflectionFormat) {
        self.inner.set("reflectionFormat", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSession {
    inner: EventTarget,
}
impl FromVal for XRSession {
    fn from_val(v: &emlite::Val) -> Self {
        XRSession {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRSession {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRSession {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRSession {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRSession> for emlite::Val {
    fn from(s: XRSession) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRSession);

impl XRSession {
    pub fn visibility_state(&self) -> XRVisibilityState {
        self.inner.get("visibilityState").as_::<XRVisibilityState>()
    }
}
impl XRSession {
    pub fn frame_rate(&self) -> f32 {
        self.inner.get("frameRate").as_::<f32>()
    }
}
impl XRSession {
    pub fn supported_frame_rates(&self) -> jsbind::Float32Array {
        self.inner
            .get("supportedFrameRates")
            .as_::<jsbind::Float32Array>()
    }
}
impl XRSession {
    pub fn render_state(&self) -> XRRenderState {
        self.inner.get("renderState").as_::<XRRenderState>()
    }
}
impl XRSession {
    pub fn input_sources(&self) -> XRInputSourceArray {
        self.inner.get("inputSources").as_::<XRInputSourceArray>()
    }
}
impl XRSession {
    pub fn tracked_sources(&self) -> XRInputSourceArray {
        self.inner.get("trackedSources").as_::<XRInputSourceArray>()
    }
}
impl XRSession {
    pub fn enabled_features(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("enabledFeatures")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl XRSession {
    pub fn is_system_keyboard_supported(&self) -> bool {
        self.inner.get("isSystemKeyboardSupported").as_::<bool>()
    }
}
impl XRSession {
    pub fn update_render_state0(&self) -> jsbind::Undefined {
        self.inner
            .call("updateRenderState", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn update_render_state1(&self, state: XRRenderStateInit) -> jsbind::Undefined {
        self.inner
            .call("updateRenderState", &[state.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl XRSession {
    pub fn update_target_frame_rate(&self, rate: f32) -> jsbind::Promise {
        self.inner
            .call("updateTargetFrameRate", &[rate.into()])
            .as_::<jsbind::Promise>()
    }
}
impl XRSession {
    pub fn request_reference_space(&self, type_: XRReferenceSpaceType) -> jsbind::Promise {
        self.inner
            .call("requestReferenceSpace", &[type_.into()])
            .as_::<jsbind::Promise>()
    }
}
impl XRSession {
    pub fn request_animation_frame(&self, callback: jsbind::Function) -> u32 {
        self.inner
            .call("requestAnimationFrame", &[callback.into()])
            .as_::<u32>()
    }
}
impl XRSession {
    pub fn cancel_animation_frame(&self, handle: u32) -> jsbind::Undefined {
        self.inner
            .call("cancelAnimationFrame", &[handle.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl XRSession {
    pub fn end(&self) -> jsbind::Promise {
        self.inner.call("end", &[]).as_::<jsbind::Promise>()
    }
}
impl XRSession {
    pub fn onend(&self) -> jsbind::Any {
        self.inner.get("onend").as_::<jsbind::Any>()
    }

    pub fn set_onend(&mut self, value: jsbind::Any) {
        self.inner.set("onend", value);
    }
}
impl XRSession {
    pub fn oninputsourceschange(&self) -> jsbind::Any {
        self.inner.get("oninputsourceschange").as_::<jsbind::Any>()
    }

    pub fn set_oninputsourceschange(&mut self, value: jsbind::Any) {
        self.inner.set("oninputsourceschange", value);
    }
}
impl XRSession {
    pub fn onselect(&self) -> jsbind::Any {
        self.inner.get("onselect").as_::<jsbind::Any>()
    }

    pub fn set_onselect(&mut self, value: jsbind::Any) {
        self.inner.set("onselect", value);
    }
}
impl XRSession {
    pub fn onselectstart(&self) -> jsbind::Any {
        self.inner.get("onselectstart").as_::<jsbind::Any>()
    }

    pub fn set_onselectstart(&mut self, value: jsbind::Any) {
        self.inner.set("onselectstart", value);
    }
}
impl XRSession {
    pub fn onselectend(&self) -> jsbind::Any {
        self.inner.get("onselectend").as_::<jsbind::Any>()
    }

    pub fn set_onselectend(&mut self, value: jsbind::Any) {
        self.inner.set("onselectend", value);
    }
}
impl XRSession {
    pub fn onsqueeze(&self) -> jsbind::Any {
        self.inner.get("onsqueeze").as_::<jsbind::Any>()
    }

    pub fn set_onsqueeze(&mut self, value: jsbind::Any) {
        self.inner.set("onsqueeze", value);
    }
}
impl XRSession {
    pub fn onsqueezestart(&self) -> jsbind::Any {
        self.inner.get("onsqueezestart").as_::<jsbind::Any>()
    }

    pub fn set_onsqueezestart(&mut self, value: jsbind::Any) {
        self.inner.set("onsqueezestart", value);
    }
}
impl XRSession {
    pub fn onsqueezeend(&self) -> jsbind::Any {
        self.inner.get("onsqueezeend").as_::<jsbind::Any>()
    }

    pub fn set_onsqueezeend(&mut self, value: jsbind::Any) {
        self.inner.set("onsqueezeend", value);
    }
}
impl XRSession {
    pub fn onvisibilitychange(&self) -> jsbind::Any {
        self.inner.get("onvisibilitychange").as_::<jsbind::Any>()
    }

    pub fn set_onvisibilitychange(&mut self, value: jsbind::Any) {
        self.inner.set("onvisibilitychange", value);
    }
}
impl XRSession {
    pub fn onframeratechange(&self) -> jsbind::Any {
        self.inner.get("onframeratechange").as_::<jsbind::Any>()
    }

    pub fn set_onframeratechange(&mut self, value: jsbind::Any) {
        self.inner.set("onframeratechange", value);
    }
}
impl XRSession {
    pub fn persistent_anchors(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("persistentAnchors")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl XRSession {
    pub fn restore_persistent_anchor(&self, uuid: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("restorePersistentAnchor", &[uuid.into()])
            .as_::<jsbind::Promise>()
    }
}
impl XRSession {
    pub fn delete_persistent_anchor(&self, uuid: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("deletePersistentAnchor", &[uuid.into()])
            .as_::<jsbind::Promise>()
    }
}
impl XRSession {
    pub fn environment_blend_mode(&self) -> XREnvironmentBlendMode {
        self.inner
            .get("environmentBlendMode")
            .as_::<XREnvironmentBlendMode>()
    }
}
impl XRSession {
    pub fn interaction_mode(&self) -> XRInteractionMode {
        self.inner.get("interactionMode").as_::<XRInteractionMode>()
    }
}
impl XRSession {
    pub fn depth_usage(&self) -> XRDepthUsage {
        self.inner.get("depthUsage").as_::<XRDepthUsage>()
    }
}
impl XRSession {
    pub fn depth_data_format(&self) -> XRDepthDataFormat {
        self.inner.get("depthDataFormat").as_::<XRDepthDataFormat>()
    }
}
impl XRSession {
    pub fn depth_type(&self) -> XRDepthType {
        self.inner.get("depthType").as_::<XRDepthType>()
    }
}
impl XRSession {
    pub fn depth_active(&self) -> bool {
        self.inner.get("depthActive").as_::<bool>()
    }
}
impl XRSession {
    pub fn pause_depth_sensing(&self) -> jsbind::Undefined {
        self.inner
            .call("pauseDepthSensing", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl XRSession {
    pub fn resume_depth_sensing(&self) -> jsbind::Undefined {
        self.inner
            .call("resumeDepthSensing", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl XRSession {
    pub fn dom_overlay_state(&self) -> XRDOMOverlayState {
        self.inner.get("domOverlayState").as_::<XRDOMOverlayState>()
    }
}
impl XRSession {
    pub fn request_hit_test_source(&self, options: XRHitTestOptionsInit) -> jsbind::Promise {
        self.inner
            .call("requestHitTestSource", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl XRSession {
    pub fn request_hit_test_source_for_transient_input(
        &self,
        options: XRTransientInputHitTestOptionsInit,
    ) -> jsbind::Promise {
        self.inner
            .call("requestHitTestSourceForTransientInput", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl XRSession {
    pub fn request_light_probe0(&self) -> jsbind::Promise {
        self.inner
            .call("requestLightProbe", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn request_light_probe1(&self, options: XRLightProbeInit) -> jsbind::Promise {
        self.inner
            .call("requestLightProbe", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl XRSession {
    pub fn preferred_reflection_format(&self) -> XRReflectionFormat {
        self.inner
            .get("preferredReflectionFormat")
            .as_::<XRReflectionFormat>()
    }
}
impl XRSession {
    pub fn initiate_room_capture(&self) -> jsbind::Promise {
        self.inner
            .call("initiateRoomCapture", &[])
            .as_::<jsbind::Promise>()
    }
}
