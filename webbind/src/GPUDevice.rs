use super::*;

/// The GPUDevice class.
/// [`GPUDevice`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUDevice {
    inner: EventTarget,
}

impl FromVal for GPUDevice {
    fn from_val(v: &Any) -> Self {
        GPUDevice {
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

impl core::ops::Deref for GPUDevice {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUDevice {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUDevice {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUDevice> for Any {
    fn from(s: GPUDevice) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUDevice> for Any {
    fn from(s: &GPUDevice) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUDevice);

impl GPUDevice {
    /// Getter of the `features` attribute.
    /// [`GPUDevice.features`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/features)
    pub fn features(&self) -> GPUSupportedFeatures {
        self.inner.get("features").as_::<GPUSupportedFeatures>()
    }
}
impl GPUDevice {
    /// Getter of the `limits` attribute.
    /// [`GPUDevice.limits`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/limits)
    pub fn limits(&self) -> GPUSupportedLimits {
        self.inner.get("limits").as_::<GPUSupportedLimits>()
    }
}
impl GPUDevice {
    /// Getter of the `adapterInfo` attribute.
    /// [`GPUDevice.adapterInfo`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/adapterInfo)
    pub fn adapter_info(&self) -> GPUAdapterInfo {
        self.inner.get("adapterInfo").as_::<GPUAdapterInfo>()
    }
}
impl GPUDevice {
    /// Getter of the `queue` attribute.
    /// [`GPUDevice.queue`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/queue)
    pub fn queue(&self) -> GPUQueue {
        self.inner.get("queue").as_::<GPUQueue>()
    }
}
impl GPUDevice {
    /// The destroy method.
    /// [`GPUDevice.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
impl GPUDevice {
    /// The createBuffer method.
    /// [`GPUDevice.createBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBuffer)
    pub fn create_buffer(&self, descriptor: &GPUBufferDescriptor) -> GPUBuffer {
        self.inner
            .call("createBuffer", &[descriptor.into()])
            .as_::<GPUBuffer>()
    }
}
impl GPUDevice {
    /// The createTexture method.
    /// [`GPUDevice.createTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createTexture)
    pub fn create_texture(&self, descriptor: &GPUTextureDescriptor) -> GPUTexture {
        self.inner
            .call("createTexture", &[descriptor.into()])
            .as_::<GPUTexture>()
    }
}
impl GPUDevice {
    /// The createSampler method.
    /// [`GPUDevice.createSampler`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)
    pub fn create_sampler0(&self) -> GPUSampler {
        self.inner.call("createSampler", &[]).as_::<GPUSampler>()
    }
    /// The createSampler method.
    /// [`GPUDevice.createSampler`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)
    pub fn create_sampler1(&self, descriptor: &GPUSamplerDescriptor) -> GPUSampler {
        self.inner
            .call("createSampler", &[descriptor.into()])
            .as_::<GPUSampler>()
    }
}
impl GPUDevice {
    /// The importExternalTexture method.
    /// [`GPUDevice.importExternalTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/importExternalTexture)
    pub fn import_external_texture(
        &self,
        descriptor: &GPUExternalTextureDescriptor,
    ) -> GPUExternalTexture {
        self.inner
            .call("importExternalTexture", &[descriptor.into()])
            .as_::<GPUExternalTexture>()
    }
}
impl GPUDevice {
    /// The createBindGroupLayout method.
    /// [`GPUDevice.createBindGroupLayout`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroupLayout)
    pub fn create_bind_group_layout(
        &self,
        descriptor: &GPUBindGroupLayoutDescriptor,
    ) -> GPUBindGroupLayout {
        self.inner
            .call("createBindGroupLayout", &[descriptor.into()])
            .as_::<GPUBindGroupLayout>()
    }
}
impl GPUDevice {
    /// The createPipelineLayout method.
    /// [`GPUDevice.createPipelineLayout`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createPipelineLayout)
    pub fn create_pipeline_layout(
        &self,
        descriptor: &GPUPipelineLayoutDescriptor,
    ) -> GPUPipelineLayout {
        self.inner
            .call("createPipelineLayout", &[descriptor.into()])
            .as_::<GPUPipelineLayout>()
    }
}
impl GPUDevice {
    /// The createBindGroup method.
    /// [`GPUDevice.createBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroup)
    pub fn create_bind_group(&self, descriptor: &GPUBindGroupDescriptor) -> GPUBindGroup {
        self.inner
            .call("createBindGroup", &[descriptor.into()])
            .as_::<GPUBindGroup>()
    }
}
impl GPUDevice {
    /// The createShaderModule method.
    /// [`GPUDevice.createShaderModule`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createShaderModule)
    pub fn create_shader_module(&self, descriptor: &GPUShaderModuleDescriptor) -> GPUShaderModule {
        self.inner
            .call("createShaderModule", &[descriptor.into()])
            .as_::<GPUShaderModule>()
    }
}
impl GPUDevice {
    /// The createComputePipeline method.
    /// [`GPUDevice.createComputePipeline`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createComputePipeline)
    pub fn create_compute_pipeline(
        &self,
        descriptor: &GPUComputePipelineDescriptor,
    ) -> GPUComputePipeline {
        self.inner
            .call("createComputePipeline", &[descriptor.into()])
            .as_::<GPUComputePipeline>()
    }
}
impl GPUDevice {
    /// The createRenderPipeline method.
    /// [`GPUDevice.createRenderPipeline`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderPipeline)
    pub fn create_render_pipeline(
        &self,
        descriptor: &GPURenderPipelineDescriptor,
    ) -> GPURenderPipeline {
        self.inner
            .call("createRenderPipeline", &[descriptor.into()])
            .as_::<GPURenderPipeline>()
    }
}
impl GPUDevice {
    /// The createComputePipelineAsync method.
    /// [`GPUDevice.createComputePipelineAsync`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createComputePipelineAsync)
    pub fn create_compute_pipeline_async(
        &self,
        descriptor: &GPUComputePipelineDescriptor,
    ) -> Promise<GPUComputePipeline> {
        self.inner
            .call("createComputePipelineAsync", &[descriptor.into()])
            .as_::<Promise<GPUComputePipeline>>()
    }
}
impl GPUDevice {
    /// The createRenderPipelineAsync method.
    /// [`GPUDevice.createRenderPipelineAsync`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderPipelineAsync)
    pub fn create_render_pipeline_async(
        &self,
        descriptor: &GPURenderPipelineDescriptor,
    ) -> Promise<GPURenderPipeline> {
        self.inner
            .call("createRenderPipelineAsync", &[descriptor.into()])
            .as_::<Promise<GPURenderPipeline>>()
    }
}
impl GPUDevice {
    /// The createCommandEncoder method.
    /// [`GPUDevice.createCommandEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)
    pub fn create_command_encoder0(&self) -> GPUCommandEncoder {
        self.inner
            .call("createCommandEncoder", &[])
            .as_::<GPUCommandEncoder>()
    }
    /// The createCommandEncoder method.
    /// [`GPUDevice.createCommandEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)
    pub fn create_command_encoder1(
        &self,
        descriptor: &GPUCommandEncoderDescriptor,
    ) -> GPUCommandEncoder {
        self.inner
            .call("createCommandEncoder", &[descriptor.into()])
            .as_::<GPUCommandEncoder>()
    }
}
impl GPUDevice {
    /// The createRenderBundleEncoder method.
    /// [`GPUDevice.createRenderBundleEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderBundleEncoder)
    pub fn create_render_bundle_encoder(
        &self,
        descriptor: &GPURenderBundleEncoderDescriptor,
    ) -> GPURenderBundleEncoder {
        self.inner
            .call("createRenderBundleEncoder", &[descriptor.into()])
            .as_::<GPURenderBundleEncoder>()
    }
}
impl GPUDevice {
    /// The createQuerySet method.
    /// [`GPUDevice.createQuerySet`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createQuerySet)
    pub fn create_query_set(&self, descriptor: &GPUQuerySetDescriptor) -> GPUQuerySet {
        self.inner
            .call("createQuerySet", &[descriptor.into()])
            .as_::<GPUQuerySet>()
    }
}
impl GPUDevice {
    /// Getter of the `lost` attribute.
    /// [`GPUDevice.lost`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/lost)
    pub fn lost(&self) -> Promise<GPUDeviceLostInfo> {
        self.inner.get("lost").as_::<Promise<GPUDeviceLostInfo>>()
    }
}
impl GPUDevice {
    /// The pushErrorScope method.
    /// [`GPUDevice.pushErrorScope`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/pushErrorScope)
    pub fn push_error_scope(&self, filter: &GPUErrorFilter) -> Undefined {
        self.inner
            .call("pushErrorScope", &[filter.into()])
            .as_::<Undefined>()
    }
}
impl GPUDevice {
    /// The popErrorScope method.
    /// [`GPUDevice.popErrorScope`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/popErrorScope)
    pub fn pop_error_scope(&self) -> Promise<GPUError> {
        self.inner
            .call("popErrorScope", &[])
            .as_::<Promise<GPUError>>()
    }
}
impl GPUDevice {
    /// Getter of the `onuncapturederror` attribute.
    /// [`GPUDevice.onuncapturederror`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)
    pub fn onuncapturederror(&self) -> Any {
        self.inner.get("onuncapturederror").as_::<Any>()
    }

    /// Setter of the `onuncapturederror` attribute.
    /// [`GPUDevice.onuncapturederror`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)
    pub fn set_onuncapturederror(&mut self, value: &Any) {
        self.inner.set("onuncapturederror", value);
    }
}
impl GPUDevice {
    /// Getter of the `label` attribute.
    /// [`GPUDevice.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUDevice.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
