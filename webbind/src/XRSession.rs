use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRRenderStateInit {
    inner: Any,
}
impl FromVal for XRRenderStateInit {
    fn from_val(v: &Any) -> Self {
        XRRenderStateInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRRenderStateInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRRenderStateInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRRenderStateInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRRenderStateInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRRenderStateInit> for Any {
    fn from(s: XRRenderStateInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRRenderStateInit> for Any {
    fn from(s: &XRRenderStateInit) -> Any {
        s.inner.clone()
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

    pub fn set_base_layer(&mut self, value: &XRWebGLLayer) {
        self.inner.set("baseLayer", value);
    }
}
impl XRRenderStateInit {
    pub fn layers(&self) -> Sequence<XRLayer> {
        self.inner.get("layers").as_::<Sequence<XRLayer>>()
    }

    pub fn set_layers(&mut self, value: &Sequence<XRLayer>) {
        self.inner.set("layers", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRDOMOverlayState {
    inner: Any,
}
impl FromVal for XRDOMOverlayState {
    fn from_val(v: &Any) -> Self {
        XRDOMOverlayState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRDOMOverlayState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRDOMOverlayState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRDOMOverlayState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRDOMOverlayState {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRDOMOverlayState> for Any {
    fn from(s: XRDOMOverlayState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRDOMOverlayState> for Any {
    fn from(s: &XRDOMOverlayState) -> Any {
        s.inner.clone()
    }
}

impl XRDOMOverlayState {
    pub fn type_(&self) -> XRDOMOverlayType {
        self.inner.get("type").as_::<XRDOMOverlayType>()
    }

    pub fn set_type_(&mut self, value: &XRDOMOverlayType) {
        self.inner.set("type", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRHitTestOptionsInit {
    inner: Any,
}
impl FromVal for XRHitTestOptionsInit {
    fn from_val(v: &Any) -> Self {
        XRHitTestOptionsInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRHitTestOptionsInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRHitTestOptionsInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRHitTestOptionsInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRHitTestOptionsInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRHitTestOptionsInit> for Any {
    fn from(s: XRHitTestOptionsInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRHitTestOptionsInit> for Any {
    fn from(s: &XRHitTestOptionsInit) -> Any {
        s.inner.clone()
    }
}

impl XRHitTestOptionsInit {
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    pub fn set_space(&mut self, value: &XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRHitTestOptionsInit {
    pub fn entity_types(&self) -> Sequence<XRHitTestTrackableType> {
        self.inner
            .get("entityTypes")
            .as_::<Sequence<XRHitTestTrackableType>>()
    }

    pub fn set_entity_types(&mut self, value: &Sequence<XRHitTestTrackableType>) {
        self.inner.set("entityTypes", value);
    }
}
impl XRHitTestOptionsInit {
    pub fn offset_ray(&self) -> XRRay {
        self.inner.get("offsetRay").as_::<XRRay>()
    }

    pub fn set_offset_ray(&mut self, value: &XRRay) {
        self.inner.set("offsetRay", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRTransientInputHitTestOptionsInit {
    inner: Any,
}
impl FromVal for XRTransientInputHitTestOptionsInit {
    fn from_val(v: &Any) -> Self {
        XRTransientInputHitTestOptionsInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRTransientInputHitTestOptionsInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRTransientInputHitTestOptionsInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRTransientInputHitTestOptionsInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRTransientInputHitTestOptionsInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRTransientInputHitTestOptionsInit> for Any {
    fn from(s: XRTransientInputHitTestOptionsInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRTransientInputHitTestOptionsInit> for Any {
    fn from(s: &XRTransientInputHitTestOptionsInit) -> Any {
        s.inner.clone()
    }
}

impl XRTransientInputHitTestOptionsInit {
    pub fn profile(&self) -> DOMString {
        self.inner.get("profile").as_::<DOMString>()
    }

    pub fn set_profile(&mut self, value: &DOMString) {
        self.inner.set("profile", value);
    }
}
impl XRTransientInputHitTestOptionsInit {
    pub fn entity_types(&self) -> Sequence<XRHitTestTrackableType> {
        self.inner
            .get("entityTypes")
            .as_::<Sequence<XRHitTestTrackableType>>()
    }

    pub fn set_entity_types(&mut self, value: &Sequence<XRHitTestTrackableType>) {
        self.inner.set("entityTypes", value);
    }
}
impl XRTransientInputHitTestOptionsInit {
    pub fn offset_ray(&self) -> XRRay {
        self.inner.get("offsetRay").as_::<XRRay>()
    }

    pub fn set_offset_ray(&mut self, value: &XRRay) {
        self.inner.set("offsetRay", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRLightProbeInit {
    inner: Any,
}
impl FromVal for XRLightProbeInit {
    fn from_val(v: &Any) -> Self {
        XRLightProbeInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRLightProbeInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRLightProbeInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRLightProbeInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRLightProbeInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRLightProbeInit> for Any {
    fn from(s: XRLightProbeInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRLightProbeInit> for Any {
    fn from(s: &XRLightProbeInit) -> Any {
        s.inner.clone()
    }
}

impl XRLightProbeInit {
    pub fn reflection_format(&self) -> XRReflectionFormat {
        self.inner
            .get("reflectionFormat")
            .as_::<XRReflectionFormat>()
    }

    pub fn set_reflection_format(&mut self, value: &XRReflectionFormat) {
        self.inner.set("reflectionFormat", value);
    }
}
/// The XRSession class.
/// [`XRSession`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSession {
    inner: EventTarget,
}
impl FromVal for XRSession {
    fn from_val(v: &Any) -> Self {
        XRSession {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for XRSession {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRSession {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRSession> for Any {
    fn from(s: XRSession) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRSession> for Any {
    fn from(s: &XRSession) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRSession);

impl XRSession {
    /// Getter of the `visibilityState` attribute.
    /// [`XRSession.visibilityState`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/visibilityState)
    pub fn visibility_state(&self) -> XRVisibilityState {
        self.inner.get("visibilityState").as_::<XRVisibilityState>()
    }
}
impl XRSession {
    /// Getter of the `frameRate` attribute.
    /// [`XRSession.frameRate`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/frameRate)
    pub fn frame_rate(&self) -> f32 {
        self.inner.get("frameRate").as_::<f32>()
    }
}
impl XRSession {
    /// Getter of the `supportedFrameRates` attribute.
    /// [`XRSession.supportedFrameRates`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/supportedFrameRates)
    pub fn supported_frame_rates(&self) -> Float32Array {
        self.inner.get("supportedFrameRates").as_::<Float32Array>()
    }
}
impl XRSession {
    /// Getter of the `renderState` attribute.
    /// [`XRSession.renderState`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/renderState)
    pub fn render_state(&self) -> XRRenderState {
        self.inner.get("renderState").as_::<XRRenderState>()
    }
}
impl XRSession {
    /// Getter of the `inputSources` attribute.
    /// [`XRSession.inputSources`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/inputSources)
    pub fn input_sources(&self) -> XRInputSourceArray {
        self.inner.get("inputSources").as_::<XRInputSourceArray>()
    }
}
impl XRSession {
    /// Getter of the `trackedSources` attribute.
    /// [`XRSession.trackedSources`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/trackedSources)
    pub fn tracked_sources(&self) -> XRInputSourceArray {
        self.inner.get("trackedSources").as_::<XRInputSourceArray>()
    }
}
impl XRSession {
    /// Getter of the `enabledFeatures` attribute.
    /// [`XRSession.enabledFeatures`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/enabledFeatures)
    pub fn enabled_features(&self) -> FrozenArray<DOMString> {
        self.inner
            .get("enabledFeatures")
            .as_::<FrozenArray<DOMString>>()
    }
}
impl XRSession {
    /// Getter of the `isSystemKeyboardSupported` attribute.
    /// [`XRSession.isSystemKeyboardSupported`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/isSystemKeyboardSupported)
    pub fn is_system_keyboard_supported(&self) -> bool {
        self.inner.get("isSystemKeyboardSupported").as_::<bool>()
    }
}
impl XRSession {
    /// The updateRenderState method.
    /// [`XRSession.updateRenderState`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/updateRenderState)
    pub fn update_render_state0(&self) -> Undefined {
        self.inner.call("updateRenderState", &[]).as_::<Undefined>()
    }
    /// The updateRenderState method.
    /// [`XRSession.updateRenderState`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/updateRenderState)
    pub fn update_render_state1(&self, state: &XRRenderStateInit) -> Undefined {
        self.inner
            .call("updateRenderState", &[state.into()])
            .as_::<Undefined>()
    }
}
impl XRSession {
    /// The updateTargetFrameRate method.
    /// [`XRSession.updateTargetFrameRate`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/updateTargetFrameRate)
    pub fn update_target_frame_rate(&self, rate: f32) -> Promise<Undefined> {
        self.inner
            .call("updateTargetFrameRate", &[rate.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl XRSession {
    /// The requestReferenceSpace method.
    /// [`XRSession.requestReferenceSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/requestReferenceSpace)
    pub fn request_reference_space(
        &self,
        type_: &XRReferenceSpaceType,
    ) -> Promise<XRReferenceSpace> {
        self.inner
            .call("requestReferenceSpace", &[type_.into()])
            .as_::<Promise<XRReferenceSpace>>()
    }
}
impl XRSession {
    /// The requestAnimationFrame method.
    /// [`XRSession.requestAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/requestAnimationFrame)
    pub fn request_animation_frame(&self, callback: &Function) -> u32 {
        self.inner
            .call("requestAnimationFrame", &[callback.into()])
            .as_::<u32>()
    }
}
impl XRSession {
    /// The cancelAnimationFrame method.
    /// [`XRSession.cancelAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/cancelAnimationFrame)
    pub fn cancel_animation_frame(&self, handle: u32) -> Undefined {
        self.inner
            .call("cancelAnimationFrame", &[handle.into()])
            .as_::<Undefined>()
    }
}
impl XRSession {
    /// The end method.
    /// [`XRSession.end`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/end)
    pub fn end(&self) -> Promise<Undefined> {
        self.inner.call("end", &[]).as_::<Promise<Undefined>>()
    }
}
impl XRSession {
    /// Getter of the `onend` attribute.
    /// [`XRSession.onend`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onend)
    pub fn onend(&self) -> Any {
        self.inner.get("onend").as_::<Any>()
    }

    /// Setter of the `onend` attribute.
    /// [`XRSession.onend`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onend)
    pub fn set_onend(&mut self, value: &Any) {
        self.inner.set("onend", value);
    }
}
impl XRSession {
    /// Getter of the `oninputsourceschange` attribute.
    /// [`XRSession.oninputsourceschange`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/oninputsourceschange)
    pub fn oninputsourceschange(&self) -> Any {
        self.inner.get("oninputsourceschange").as_::<Any>()
    }

    /// Setter of the `oninputsourceschange` attribute.
    /// [`XRSession.oninputsourceschange`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/oninputsourceschange)
    pub fn set_oninputsourceschange(&mut self, value: &Any) {
        self.inner.set("oninputsourceschange", value);
    }
}
impl XRSession {
    /// Getter of the `onselect` attribute.
    /// [`XRSession.onselect`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onselect)
    pub fn onselect(&self) -> Any {
        self.inner.get("onselect").as_::<Any>()
    }

    /// Setter of the `onselect` attribute.
    /// [`XRSession.onselect`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onselect)
    pub fn set_onselect(&mut self, value: &Any) {
        self.inner.set("onselect", value);
    }
}
impl XRSession {
    /// Getter of the `onselectstart` attribute.
    /// [`XRSession.onselectstart`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onselectstart)
    pub fn onselectstart(&self) -> Any {
        self.inner.get("onselectstart").as_::<Any>()
    }

    /// Setter of the `onselectstart` attribute.
    /// [`XRSession.onselectstart`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onselectstart)
    pub fn set_onselectstart(&mut self, value: &Any) {
        self.inner.set("onselectstart", value);
    }
}
impl XRSession {
    /// Getter of the `onselectend` attribute.
    /// [`XRSession.onselectend`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onselectend)
    pub fn onselectend(&self) -> Any {
        self.inner.get("onselectend").as_::<Any>()
    }

    /// Setter of the `onselectend` attribute.
    /// [`XRSession.onselectend`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onselectend)
    pub fn set_onselectend(&mut self, value: &Any) {
        self.inner.set("onselectend", value);
    }
}
impl XRSession {
    /// Getter of the `onsqueeze` attribute.
    /// [`XRSession.onsqueeze`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onsqueeze)
    pub fn onsqueeze(&self) -> Any {
        self.inner.get("onsqueeze").as_::<Any>()
    }

    /// Setter of the `onsqueeze` attribute.
    /// [`XRSession.onsqueeze`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onsqueeze)
    pub fn set_onsqueeze(&mut self, value: &Any) {
        self.inner.set("onsqueeze", value);
    }
}
impl XRSession {
    /// Getter of the `onsqueezestart` attribute.
    /// [`XRSession.onsqueezestart`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onsqueezestart)
    pub fn onsqueezestart(&self) -> Any {
        self.inner.get("onsqueezestart").as_::<Any>()
    }

    /// Setter of the `onsqueezestart` attribute.
    /// [`XRSession.onsqueezestart`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onsqueezestart)
    pub fn set_onsqueezestart(&mut self, value: &Any) {
        self.inner.set("onsqueezestart", value);
    }
}
impl XRSession {
    /// Getter of the `onsqueezeend` attribute.
    /// [`XRSession.onsqueezeend`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onsqueezeend)
    pub fn onsqueezeend(&self) -> Any {
        self.inner.get("onsqueezeend").as_::<Any>()
    }

    /// Setter of the `onsqueezeend` attribute.
    /// [`XRSession.onsqueezeend`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onsqueezeend)
    pub fn set_onsqueezeend(&mut self, value: &Any) {
        self.inner.set("onsqueezeend", value);
    }
}
impl XRSession {
    /// Getter of the `onvisibilitychange` attribute.
    /// [`XRSession.onvisibilitychange`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onvisibilitychange)
    pub fn onvisibilitychange(&self) -> Any {
        self.inner.get("onvisibilitychange").as_::<Any>()
    }

    /// Setter of the `onvisibilitychange` attribute.
    /// [`XRSession.onvisibilitychange`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onvisibilitychange)
    pub fn set_onvisibilitychange(&mut self, value: &Any) {
        self.inner.set("onvisibilitychange", value);
    }
}
impl XRSession {
    /// Getter of the `onframeratechange` attribute.
    /// [`XRSession.onframeratechange`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onframeratechange)
    pub fn onframeratechange(&self) -> Any {
        self.inner.get("onframeratechange").as_::<Any>()
    }

    /// Setter of the `onframeratechange` attribute.
    /// [`XRSession.onframeratechange`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/onframeratechange)
    pub fn set_onframeratechange(&mut self, value: &Any) {
        self.inner.set("onframeratechange", value);
    }
}
impl XRSession {
    /// Getter of the `persistentAnchors` attribute.
    /// [`XRSession.persistentAnchors`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/persistentAnchors)
    pub fn persistent_anchors(&self) -> FrozenArray<DOMString> {
        self.inner
            .get("persistentAnchors")
            .as_::<FrozenArray<DOMString>>()
    }
}
impl XRSession {
    /// The restorePersistentAnchor method.
    /// [`XRSession.restorePersistentAnchor`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/restorePersistentAnchor)
    pub fn restore_persistent_anchor(&self, uuid: &DOMString) -> Promise<XRAnchor> {
        self.inner
            .call("restorePersistentAnchor", &[uuid.into()])
            .as_::<Promise<XRAnchor>>()
    }
}
impl XRSession {
    /// The deletePersistentAnchor method.
    /// [`XRSession.deletePersistentAnchor`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/deletePersistentAnchor)
    pub fn delete_persistent_anchor(&self, uuid: &DOMString) -> Promise<Undefined> {
        self.inner
            .call("deletePersistentAnchor", &[uuid.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl XRSession {
    /// Getter of the `environmentBlendMode` attribute.
    /// [`XRSession.environmentBlendMode`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/environmentBlendMode)
    pub fn environment_blend_mode(&self) -> XREnvironmentBlendMode {
        self.inner
            .get("environmentBlendMode")
            .as_::<XREnvironmentBlendMode>()
    }
}
impl XRSession {
    /// Getter of the `interactionMode` attribute.
    /// [`XRSession.interactionMode`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/interactionMode)
    pub fn interaction_mode(&self) -> XRInteractionMode {
        self.inner.get("interactionMode").as_::<XRInteractionMode>()
    }
}
impl XRSession {
    /// Getter of the `depthUsage` attribute.
    /// [`XRSession.depthUsage`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/depthUsage)
    pub fn depth_usage(&self) -> XRDepthUsage {
        self.inner.get("depthUsage").as_::<XRDepthUsage>()
    }
}
impl XRSession {
    /// Getter of the `depthDataFormat` attribute.
    /// [`XRSession.depthDataFormat`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/depthDataFormat)
    pub fn depth_data_format(&self) -> XRDepthDataFormat {
        self.inner.get("depthDataFormat").as_::<XRDepthDataFormat>()
    }
}
impl XRSession {
    /// Getter of the `depthType` attribute.
    /// [`XRSession.depthType`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/depthType)
    pub fn depth_type(&self) -> XRDepthType {
        self.inner.get("depthType").as_::<XRDepthType>()
    }
}
impl XRSession {
    /// Getter of the `depthActive` attribute.
    /// [`XRSession.depthActive`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/depthActive)
    pub fn depth_active(&self) -> bool {
        self.inner.get("depthActive").as_::<bool>()
    }
}
impl XRSession {
    /// The pauseDepthSensing method.
    /// [`XRSession.pauseDepthSensing`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/pauseDepthSensing)
    pub fn pause_depth_sensing(&self) -> Undefined {
        self.inner.call("pauseDepthSensing", &[]).as_::<Undefined>()
    }
}
impl XRSession {
    /// The resumeDepthSensing method.
    /// [`XRSession.resumeDepthSensing`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/resumeDepthSensing)
    pub fn resume_depth_sensing(&self) -> Undefined {
        self.inner
            .call("resumeDepthSensing", &[])
            .as_::<Undefined>()
    }
}
impl XRSession {
    /// Getter of the `domOverlayState` attribute.
    /// [`XRSession.domOverlayState`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/domOverlayState)
    pub fn dom_overlay_state(&self) -> XRDOMOverlayState {
        self.inner.get("domOverlayState").as_::<XRDOMOverlayState>()
    }
}
impl XRSession {
    /// The requestHitTestSource method.
    /// [`XRSession.requestHitTestSource`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/requestHitTestSource)
    pub fn request_hit_test_source(
        &self,
        options: &XRHitTestOptionsInit,
    ) -> Promise<XRHitTestSource> {
        self.inner
            .call("requestHitTestSource", &[options.into()])
            .as_::<Promise<XRHitTestSource>>()
    }
}
impl XRSession {
    /// The requestHitTestSourceForTransientInput method.
    /// [`XRSession.requestHitTestSourceForTransientInput`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/requestHitTestSourceForTransientInput)
    pub fn request_hit_test_source_for_transient_input(
        &self,
        options: &XRTransientInputHitTestOptionsInit,
    ) -> Promise<XRTransientInputHitTestSource> {
        self.inner
            .call("requestHitTestSourceForTransientInput", &[options.into()])
            .as_::<Promise<XRTransientInputHitTestSource>>()
    }
}
impl XRSession {
    /// The requestLightProbe method.
    /// [`XRSession.requestLightProbe`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/requestLightProbe)
    pub fn request_light_probe0(&self) -> Promise<XRLightProbe> {
        self.inner
            .call("requestLightProbe", &[])
            .as_::<Promise<XRLightProbe>>()
    }
    /// The requestLightProbe method.
    /// [`XRSession.requestLightProbe`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/requestLightProbe)
    pub fn request_light_probe1(&self, options: &XRLightProbeInit) -> Promise<XRLightProbe> {
        self.inner
            .call("requestLightProbe", &[options.into()])
            .as_::<Promise<XRLightProbe>>()
    }
}
impl XRSession {
    /// Getter of the `preferredReflectionFormat` attribute.
    /// [`XRSession.preferredReflectionFormat`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/preferredReflectionFormat)
    pub fn preferred_reflection_format(&self) -> XRReflectionFormat {
        self.inner
            .get("preferredReflectionFormat")
            .as_::<XRReflectionFormat>()
    }
}
impl XRSession {
    /// The initiateRoomCapture method.
    /// [`XRSession.initiateRoomCapture`](https://developer.mozilla.org/en-US/docs/Web/API/XRSession/initiateRoomCapture)
    pub fn initiate_room_capture(&self) -> Promise<Undefined> {
        self.inner
            .call("initiateRoomCapture", &[])
            .as_::<Promise<Undefined>>()
    }
}
