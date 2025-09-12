use super::*;

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
