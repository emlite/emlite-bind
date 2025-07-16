use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRProjectionLayerInit {
    inner: Any,
}
impl FromVal for XRProjectionLayerInit {
    fn from_val(v: &Any) -> Self {
        XRProjectionLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRProjectionLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRProjectionLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRProjectionLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRProjectionLayerInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRProjectionLayerInit> for Any {
    fn from(s: XRProjectionLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRProjectionLayerInit> for Any {
    fn from(s: &XRProjectionLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRProjectionLayerInit {
    pub fn texture_type(&self) -> XRTextureType {
        self.inner.get("textureType").as_::<XRTextureType>()
    }

    pub fn set_texture_type(&mut self, value: &XRTextureType) {
        self.inner.set("textureType", value);
    }
}
impl XRProjectionLayerInit {
    pub fn color_format(&self) -> Any {
        self.inner.get("colorFormat").as_::<Any>()
    }

    pub fn set_color_format(&mut self, value: &Any) {
        self.inner.set("colorFormat", value);
    }
}
impl XRProjectionLayerInit {
    pub fn depth_format(&self) -> Any {
        self.inner.get("depthFormat").as_::<Any>()
    }

    pub fn set_depth_format(&mut self, value: &Any) {
        self.inner.set("depthFormat", value);
    }
}
impl XRProjectionLayerInit {
    pub fn scale_factor(&self) -> f64 {
        self.inner.get("scaleFactor").as_::<f64>()
    }

    pub fn set_scale_factor(&mut self, value: f64) {
        self.inner.set("scaleFactor", value);
    }
}
impl XRProjectionLayerInit {
    pub fn clear_on_access(&self) -> bool {
        self.inner.get("clearOnAccess").as_::<bool>()
    }

    pub fn set_clear_on_access(&mut self, value: bool) {
        self.inner.set("clearOnAccess", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRQuadLayerInit {
    inner: Any,
}
impl FromVal for XRQuadLayerInit {
    fn from_val(v: &Any) -> Self {
        XRQuadLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRQuadLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRQuadLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRQuadLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRQuadLayerInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRQuadLayerInit> for Any {
    fn from(s: XRQuadLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRQuadLayerInit> for Any {
    fn from(s: &XRQuadLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRQuadLayerInit {
    pub fn texture_type(&self) -> XRTextureType {
        self.inner.get("textureType").as_::<XRTextureType>()
    }

    pub fn set_texture_type(&mut self, value: &XRTextureType) {
        self.inner.set("textureType", value);
    }
}
impl XRQuadLayerInit {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    pub fn set_transform(&mut self, value: &XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRQuadLayerInit {
    pub fn width(&self) -> f32 {
        self.inner.get("width").as_::<f32>()
    }

    pub fn set_width(&mut self, value: f32) {
        self.inner.set("width", value);
    }
}
impl XRQuadLayerInit {
    pub fn height(&self) -> f32 {
        self.inner.get("height").as_::<f32>()
    }

    pub fn set_height(&mut self, value: f32) {
        self.inner.set("height", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRCylinderLayerInit {
    inner: Any,
}
impl FromVal for XRCylinderLayerInit {
    fn from_val(v: &Any) -> Self {
        XRCylinderLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRCylinderLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRCylinderLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRCylinderLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRCylinderLayerInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRCylinderLayerInit> for Any {
    fn from(s: XRCylinderLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRCylinderLayerInit> for Any {
    fn from(s: &XRCylinderLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRCylinderLayerInit {
    pub fn texture_type(&self) -> XRTextureType {
        self.inner.get("textureType").as_::<XRTextureType>()
    }

    pub fn set_texture_type(&mut self, value: &XRTextureType) {
        self.inner.set("textureType", value);
    }
}
impl XRCylinderLayerInit {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    pub fn set_transform(&mut self, value: &XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XRCylinderLayerInit {
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }

    pub fn set_radius(&mut self, value: f32) {
        self.inner.set("radius", value);
    }
}
impl XRCylinderLayerInit {
    pub fn central_angle(&self) -> f32 {
        self.inner.get("centralAngle").as_::<f32>()
    }

    pub fn set_central_angle(&mut self, value: f32) {
        self.inner.set("centralAngle", value);
    }
}
impl XRCylinderLayerInit {
    pub fn aspect_ratio(&self) -> f32 {
        self.inner.get("aspectRatio").as_::<f32>()
    }

    pub fn set_aspect_ratio(&mut self, value: f32) {
        self.inner.set("aspectRatio", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XREquirectLayerInit {
    inner: Any,
}
impl FromVal for XREquirectLayerInit {
    fn from_val(v: &Any) -> Self {
        XREquirectLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XREquirectLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XREquirectLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XREquirectLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XREquirectLayerInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XREquirectLayerInit> for Any {
    fn from(s: XREquirectLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XREquirectLayerInit> for Any {
    fn from(s: &XREquirectLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XREquirectLayerInit {
    pub fn texture_type(&self) -> XRTextureType {
        self.inner.get("textureType").as_::<XRTextureType>()
    }

    pub fn set_texture_type(&mut self, value: &XRTextureType) {
        self.inner.set("textureType", value);
    }
}
impl XREquirectLayerInit {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    pub fn set_transform(&mut self, value: &XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
impl XREquirectLayerInit {
    pub fn radius(&self) -> f32 {
        self.inner.get("radius").as_::<f32>()
    }

    pub fn set_radius(&mut self, value: f32) {
        self.inner.set("radius", value);
    }
}
impl XREquirectLayerInit {
    pub fn central_horizontal_angle(&self) -> f32 {
        self.inner.get("centralHorizontalAngle").as_::<f32>()
    }

    pub fn set_central_horizontal_angle(&mut self, value: f32) {
        self.inner.set("centralHorizontalAngle", value);
    }
}
impl XREquirectLayerInit {
    pub fn upper_vertical_angle(&self) -> f32 {
        self.inner.get("upperVerticalAngle").as_::<f32>()
    }

    pub fn set_upper_vertical_angle(&mut self, value: f32) {
        self.inner.set("upperVerticalAngle", value);
    }
}
impl XREquirectLayerInit {
    pub fn lower_vertical_angle(&self) -> f32 {
        self.inner.get("lowerVerticalAngle").as_::<f32>()
    }

    pub fn set_lower_vertical_angle(&mut self, value: f32) {
        self.inner.set("lowerVerticalAngle", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRCubeLayerInit {
    inner: Any,
}
impl FromVal for XRCubeLayerInit {
    fn from_val(v: &Any) -> Self {
        XRCubeLayerInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRCubeLayerInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRCubeLayerInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRCubeLayerInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRCubeLayerInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRCubeLayerInit> for Any {
    fn from(s: XRCubeLayerInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRCubeLayerInit> for Any {
    fn from(s: &XRCubeLayerInit) -> Any {
        s.inner.clone()
    }
}

impl XRCubeLayerInit {
    pub fn orientation(&self) -> DOMPointReadOnly {
        self.inner.get("orientation").as_::<DOMPointReadOnly>()
    }

    pub fn set_orientation(&mut self, value: &DOMPointReadOnly) {
        self.inner.set("orientation", value);
    }
}
/// The XRWebGLBinding class.
/// [`XRWebGLBinding`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRWebGLBinding {
    inner: Any,
}
impl FromVal for XRWebGLBinding {
    fn from_val(v: &Any) -> Self {
        XRWebGLBinding {
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
impl core::ops::Deref for XRWebGLBinding {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRWebGLBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRWebGLBinding {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRWebGLBinding {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRWebGLBinding> for Any {
    fn from(s: XRWebGLBinding) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRWebGLBinding> for Any {
    fn from(s: &XRWebGLBinding) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRWebGLBinding);

impl XRWebGLBinding {
    /// The `new XRWebGLBinding(..)` constructor, creating a new XRWebGLBinding instance
    pub fn new(session: &XRSession, context: &Any) -> XRWebGLBinding {
        Self {
            inner: Any::global("XRWebGLBinding")
                .new(&[session.into(), context.into()])
                .as_::<Any>(),
        }
    }
}
impl XRWebGLBinding {
    /// Getter of the `nativeProjectionScaleFactor` attribute.
    /// [`XRWebGLBinding.nativeProjectionScaleFactor`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/nativeProjectionScaleFactor)
    pub fn native_projection_scale_factor(&self) -> f64 {
        self.inner.get("nativeProjectionScaleFactor").as_::<f64>()
    }
}
impl XRWebGLBinding {
    /// Getter of the `usesDepthValues` attribute.
    /// [`XRWebGLBinding.usesDepthValues`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/usesDepthValues)
    pub fn uses_depth_values(&self) -> bool {
        self.inner.get("usesDepthValues").as_::<bool>()
    }
}
impl XRWebGLBinding {
    /// The createProjectionLayer method.
    /// [`XRWebGLBinding.createProjectionLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/createProjectionLayer)
    pub fn create_projection_layer0(&self) -> XRProjectionLayer {
        self.inner
            .call("createProjectionLayer", &[])
            .as_::<XRProjectionLayer>()
    }
    /// The createProjectionLayer method.
    /// [`XRWebGLBinding.createProjectionLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/createProjectionLayer)
    pub fn create_projection_layer1(&self, init: &XRProjectionLayerInit) -> XRProjectionLayer {
        self.inner
            .call("createProjectionLayer", &[init.into()])
            .as_::<XRProjectionLayer>()
    }
}
impl XRWebGLBinding {
    /// The createQuadLayer method.
    /// [`XRWebGLBinding.createQuadLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/createQuadLayer)
    pub fn create_quad_layer0(&self) -> XRQuadLayer {
        self.inner.call("createQuadLayer", &[]).as_::<XRQuadLayer>()
    }
    /// The createQuadLayer method.
    /// [`XRWebGLBinding.createQuadLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/createQuadLayer)
    pub fn create_quad_layer1(&self, init: &XRQuadLayerInit) -> XRQuadLayer {
        self.inner
            .call("createQuadLayer", &[init.into()])
            .as_::<XRQuadLayer>()
    }
}
impl XRWebGLBinding {
    /// The createCylinderLayer method.
    /// [`XRWebGLBinding.createCylinderLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/createCylinderLayer)
    pub fn create_cylinder_layer0(&self) -> XRCylinderLayer {
        self.inner
            .call("createCylinderLayer", &[])
            .as_::<XRCylinderLayer>()
    }
    /// The createCylinderLayer method.
    /// [`XRWebGLBinding.createCylinderLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/createCylinderLayer)
    pub fn create_cylinder_layer1(&self, init: &XRCylinderLayerInit) -> XRCylinderLayer {
        self.inner
            .call("createCylinderLayer", &[init.into()])
            .as_::<XRCylinderLayer>()
    }
}
impl XRWebGLBinding {
    /// The createEquirectLayer method.
    /// [`XRWebGLBinding.createEquirectLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/createEquirectLayer)
    pub fn create_equirect_layer0(&self) -> XREquirectLayer {
        self.inner
            .call("createEquirectLayer", &[])
            .as_::<XREquirectLayer>()
    }
    /// The createEquirectLayer method.
    /// [`XRWebGLBinding.createEquirectLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/createEquirectLayer)
    pub fn create_equirect_layer1(&self, init: &XREquirectLayerInit) -> XREquirectLayer {
        self.inner
            .call("createEquirectLayer", &[init.into()])
            .as_::<XREquirectLayer>()
    }
}
impl XRWebGLBinding {
    /// The createCubeLayer method.
    /// [`XRWebGLBinding.createCubeLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/createCubeLayer)
    pub fn create_cube_layer0(&self) -> XRCubeLayer {
        self.inner.call("createCubeLayer", &[]).as_::<XRCubeLayer>()
    }
    /// The createCubeLayer method.
    /// [`XRWebGLBinding.createCubeLayer`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/createCubeLayer)
    pub fn create_cube_layer1(&self, init: &XRCubeLayerInit) -> XRCubeLayer {
        self.inner
            .call("createCubeLayer", &[init.into()])
            .as_::<XRCubeLayer>()
    }
}
impl XRWebGLBinding {
    /// The getSubImage method.
    /// [`XRWebGLBinding.getSubImage`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/getSubImage)
    pub fn get_sub_image0(&self, layer: &XRCompositionLayer, frame: &XRFrame) -> XRWebGLSubImage {
        self.inner
            .call("getSubImage", &[layer.into(), frame.into()])
            .as_::<XRWebGLSubImage>()
    }
    /// The getSubImage method.
    /// [`XRWebGLBinding.getSubImage`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/getSubImage)
    pub fn get_sub_image1(
        &self,
        layer: &XRCompositionLayer,
        frame: &XRFrame,
        eye: &XREye,
    ) -> XRWebGLSubImage {
        self.inner
            .call("getSubImage", &[layer.into(), frame.into(), eye.into()])
            .as_::<XRWebGLSubImage>()
    }
}
impl XRWebGLBinding {
    /// The getViewSubImage method.
    /// [`XRWebGLBinding.getViewSubImage`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/getViewSubImage)
    pub fn get_view_sub_image(&self, layer: &XRProjectionLayer, view: &XRView) -> XRWebGLSubImage {
        self.inner
            .call("getViewSubImage", &[layer.into(), view.into()])
            .as_::<XRWebGLSubImage>()
    }
}
impl XRWebGLBinding {
    /// The foveateBoundTexture method.
    /// [`XRWebGLBinding.foveateBoundTexture`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/foveateBoundTexture)
    pub fn foveate_bound_texture(&self, target: &Any, fixed_foveation: f32) -> Undefined {
        self.inner
            .call(
                "foveateBoundTexture",
                &[target.into(), fixed_foveation.into()],
            )
            .as_::<Undefined>()
    }
}
impl XRWebGLBinding {
    /// The getCameraImage method.
    /// [`XRWebGLBinding.getCameraImage`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/getCameraImage)
    pub fn get_camera_image(&self, camera: &XRCamera) -> WebGLTexture {
        self.inner
            .call("getCameraImage", &[camera.into()])
            .as_::<WebGLTexture>()
    }
}
impl XRWebGLBinding {
    /// The getDepthInformation method.
    /// [`XRWebGLBinding.getDepthInformation`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/getDepthInformation)
    pub fn get_depth_information(&self, view: &XRView) -> XRWebGLDepthInformation {
        self.inner
            .call("getDepthInformation", &[view.into()])
            .as_::<XRWebGLDepthInformation>()
    }
}
impl XRWebGLBinding {
    /// The getReflectionCubeMap method.
    /// [`XRWebGLBinding.getReflectionCubeMap`](https://developer.mozilla.org/en-US/docs/Web/API/XRWebGLBinding/getReflectionCubeMap)
    pub fn get_reflection_cube_map(&self, light_probe: &XRLightProbe) -> WebGLTexture {
        self.inner
            .call("getReflectionCubeMap", &[light_probe.into()])
            .as_::<WebGLTexture>()
    }
}
