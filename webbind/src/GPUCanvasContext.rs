use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCanvasConfiguration {
    inner: Any,
}
impl FromVal for GPUCanvasConfiguration {
    fn from_val(v: &Any) -> Self {
        GPUCanvasConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCanvasConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCanvasConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUCanvasConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUCanvasConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUCanvasConfiguration> for Any {
    fn from(s: GPUCanvasConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUCanvasConfiguration> for Any {
    fn from(s: &GPUCanvasConfiguration) -> Any {
        s.inner.clone()
    }
}

impl GPUCanvasConfiguration {
    pub fn device(&self) -> GPUDevice {
        self.inner.get("device").as_::<GPUDevice>()
    }

    pub fn set_device(&mut self, value: &GPUDevice) {
        self.inner.set("device", value);
    }
}
impl GPUCanvasConfiguration {
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    pub fn set_format(&mut self, value: &GPUTextureFormat) {
        self.inner.set("format", value);
    }
}
impl GPUCanvasConfiguration {
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }

    pub fn set_usage(&mut self, value: &Any) {
        self.inner.set("usage", value);
    }
}
impl GPUCanvasConfiguration {
    pub fn view_formats(&self) -> TypedArray<GPUTextureFormat> {
        self.inner
            .get("viewFormats")
            .as_::<TypedArray<GPUTextureFormat>>()
    }

    pub fn set_view_formats(&mut self, value: &TypedArray<GPUTextureFormat>) {
        self.inner.set("viewFormats", value);
    }
}
impl GPUCanvasConfiguration {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
impl GPUCanvasConfiguration {
    pub fn tone_mapping(&self) -> Any {
        self.inner.get("toneMapping").as_::<Any>()
    }

    pub fn set_tone_mapping(&mut self, value: &Any) {
        self.inner.set("toneMapping", value);
    }
}
impl GPUCanvasConfiguration {
    pub fn alpha_mode(&self) -> GPUCanvasAlphaMode {
        self.inner.get("alphaMode").as_::<GPUCanvasAlphaMode>()
    }

    pub fn set_alpha_mode(&mut self, value: &GPUCanvasAlphaMode) {
        self.inner.set("alphaMode", value);
    }
}
/// The GPUCanvasContext class.
/// [`GPUCanvasContext`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCanvasContext {
    inner: Any,
}
impl FromVal for GPUCanvasContext {
    fn from_val(v: &Any) -> Self {
        GPUCanvasContext {
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
impl core::ops::Deref for GPUCanvasContext {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCanvasContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUCanvasContext {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUCanvasContext {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUCanvasContext> for Any {
    fn from(s: GPUCanvasContext) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUCanvasContext> for Any {
    fn from(s: &GPUCanvasContext) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUCanvasContext);

impl GPUCanvasContext {
    /// Getter of the `canvas` attribute.
    /// [`GPUCanvasContext.canvas`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/canvas)
    pub fn canvas(&self) -> Any {
        self.inner.get("canvas").as_::<Any>()
    }
}
impl GPUCanvasContext {
    /// The configure method.
    /// [`GPUCanvasContext.configure`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/configure)
    pub fn configure(&self, configuration: &GPUCanvasConfiguration) -> Undefined {
        self.inner
            .call("configure", &[configuration.into()])
            .as_::<Undefined>()
    }
}
impl GPUCanvasContext {
    /// The unconfigure method.
    /// [`GPUCanvasContext.unconfigure`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/unconfigure)
    pub fn unconfigure(&self) -> Undefined {
        self.inner.call("unconfigure", &[]).as_::<Undefined>()
    }
}
impl GPUCanvasContext {
    /// The getConfiguration method.
    /// [`GPUCanvasContext.getConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/getConfiguration)
    pub fn get_configuration(&self) -> GPUCanvasConfiguration {
        self.inner
            .call("getConfiguration", &[])
            .as_::<GPUCanvasConfiguration>()
    }
}
impl GPUCanvasContext {
    /// The getCurrentTexture method.
    /// [`GPUCanvasContext.getCurrentTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/getCurrentTexture)
    pub fn get_current_texture(&self) -> GPUTexture {
        self.inner
            .call("getCurrentTexture", &[])
            .as_::<GPUTexture>()
    }
}
