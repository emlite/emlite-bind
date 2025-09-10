use super::*;

/// The GPUSupportedLimits class.
/// [`GPUSupportedLimits`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUSupportedLimits {
    inner: Any,
}

impl FromVal for GPUSupportedLimits {
    fn from_val(v: &Any) -> Self {
        GPUSupportedLimits {
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

impl core::ops::Deref for GPUSupportedLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUSupportedLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUSupportedLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUSupportedLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUSupportedLimits> for Any {
    fn from(s: GPUSupportedLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUSupportedLimits> for Any {
    fn from(s: &GPUSupportedLimits) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUSupportedLimits);

impl GPUSupportedLimits {
    /// Getter of the `maxTextureDimension1D` attribute.
    /// [`GPUSupportedLimits.maxTextureDimension1D`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureDimension1D)
    pub fn max_texture_dimension1_d(&self) -> u32 {
        self.inner.get("maxTextureDimension1D").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxTextureDimension2D` attribute.
    /// [`GPUSupportedLimits.maxTextureDimension2D`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureDimension2D)
    pub fn max_texture_dimension2_d(&self) -> u32 {
        self.inner.get("maxTextureDimension2D").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxTextureDimension3D` attribute.
    /// [`GPUSupportedLimits.maxTextureDimension3D`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureDimension3D)
    pub fn max_texture_dimension3_d(&self) -> u32 {
        self.inner.get("maxTextureDimension3D").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxTextureArrayLayers` attribute.
    /// [`GPUSupportedLimits.maxTextureArrayLayers`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureArrayLayers)
    pub fn max_texture_array_layers(&self) -> u32 {
        self.inner.get("maxTextureArrayLayers").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxBindGroups` attribute.
    /// [`GPUSupportedLimits.maxBindGroups`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBindGroups)
    pub fn max_bind_groups(&self) -> u32 {
        self.inner.get("maxBindGroups").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxBindGroupsPlusVertexBuffers` attribute.
    /// [`GPUSupportedLimits.maxBindGroupsPlusVertexBuffers`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBindGroupsPlusVertexBuffers)
    pub fn max_bind_groups_plus_vertex_buffers(&self) -> u32 {
        self.inner
            .get("maxBindGroupsPlusVertexBuffers")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxBindingsPerBindGroup` attribute.
    /// [`GPUSupportedLimits.maxBindingsPerBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBindingsPerBindGroup)
    pub fn max_bindings_per_bind_group(&self) -> u32 {
        self.inner.get("maxBindingsPerBindGroup").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxDynamicUniformBuffersPerPipelineLayout` attribute.
    /// [`GPUSupportedLimits.maxDynamicUniformBuffersPerPipelineLayout`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxDynamicUniformBuffersPerPipelineLayout)
    pub fn max_dynamic_uniform_buffers_per_pipeline_layout(&self) -> u32 {
        self.inner
            .get("maxDynamicUniformBuffersPerPipelineLayout")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxDynamicStorageBuffersPerPipelineLayout` attribute.
    /// [`GPUSupportedLimits.maxDynamicStorageBuffersPerPipelineLayout`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxDynamicStorageBuffersPerPipelineLayout)
    pub fn max_dynamic_storage_buffers_per_pipeline_layout(&self) -> u32 {
        self.inner
            .get("maxDynamicStorageBuffersPerPipelineLayout")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxSampledTexturesPerShaderStage` attribute.
    /// [`GPUSupportedLimits.maxSampledTexturesPerShaderStage`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxSampledTexturesPerShaderStage)
    pub fn max_sampled_textures_per_shader_stage(&self) -> u32 {
        self.inner
            .get("maxSampledTexturesPerShaderStage")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxSamplersPerShaderStage` attribute.
    /// [`GPUSupportedLimits.maxSamplersPerShaderStage`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxSamplersPerShaderStage)
    pub fn max_samplers_per_shader_stage(&self) -> u32 {
        self.inner.get("maxSamplersPerShaderStage").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxStorageBuffersPerShaderStage` attribute.
    /// [`GPUSupportedLimits.maxStorageBuffersPerShaderStage`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageBuffersPerShaderStage)
    pub fn max_storage_buffers_per_shader_stage(&self) -> u32 {
        self.inner
            .get("maxStorageBuffersPerShaderStage")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxStorageTexturesPerShaderStage` attribute.
    /// [`GPUSupportedLimits.maxStorageTexturesPerShaderStage`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageTexturesPerShaderStage)
    pub fn max_storage_textures_per_shader_stage(&self) -> u32 {
        self.inner
            .get("maxStorageTexturesPerShaderStage")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxUniformBuffersPerShaderStage` attribute.
    /// [`GPUSupportedLimits.maxUniformBuffersPerShaderStage`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxUniformBuffersPerShaderStage)
    pub fn max_uniform_buffers_per_shader_stage(&self) -> u32 {
        self.inner
            .get("maxUniformBuffersPerShaderStage")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxUniformBufferBindingSize` attribute.
    /// [`GPUSupportedLimits.maxUniformBufferBindingSize`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxUniformBufferBindingSize)
    pub fn max_uniform_buffer_binding_size(&self) -> u64 {
        self.inner.get("maxUniformBufferBindingSize").as_::<u64>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxStorageBufferBindingSize` attribute.
    /// [`GPUSupportedLimits.maxStorageBufferBindingSize`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageBufferBindingSize)
    pub fn max_storage_buffer_binding_size(&self) -> u64 {
        self.inner.get("maxStorageBufferBindingSize").as_::<u64>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `minUniformBufferOffsetAlignment` attribute.
    /// [`GPUSupportedLimits.minUniformBufferOffsetAlignment`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/minUniformBufferOffsetAlignment)
    pub fn min_uniform_buffer_offset_alignment(&self) -> u32 {
        self.inner
            .get("minUniformBufferOffsetAlignment")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `minStorageBufferOffsetAlignment` attribute.
    /// [`GPUSupportedLimits.minStorageBufferOffsetAlignment`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/minStorageBufferOffsetAlignment)
    pub fn min_storage_buffer_offset_alignment(&self) -> u32 {
        self.inner
            .get("minStorageBufferOffsetAlignment")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxVertexBuffers` attribute.
    /// [`GPUSupportedLimits.maxVertexBuffers`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxVertexBuffers)
    pub fn max_vertex_buffers(&self) -> u32 {
        self.inner.get("maxVertexBuffers").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxBufferSize` attribute.
    /// [`GPUSupportedLimits.maxBufferSize`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBufferSize)
    pub fn max_buffer_size(&self) -> u64 {
        self.inner.get("maxBufferSize").as_::<u64>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxVertexAttributes` attribute.
    /// [`GPUSupportedLimits.maxVertexAttributes`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxVertexAttributes)
    pub fn max_vertex_attributes(&self) -> u32 {
        self.inner.get("maxVertexAttributes").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxVertexBufferArrayStride` attribute.
    /// [`GPUSupportedLimits.maxVertexBufferArrayStride`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxVertexBufferArrayStride)
    pub fn max_vertex_buffer_array_stride(&self) -> u32 {
        self.inner.get("maxVertexBufferArrayStride").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxInterStageShaderVariables` attribute.
    /// [`GPUSupportedLimits.maxInterStageShaderVariables`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxInterStageShaderVariables)
    pub fn max_inter_stage_shader_variables(&self) -> u32 {
        self.inner.get("maxInterStageShaderVariables").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxColorAttachments` attribute.
    /// [`GPUSupportedLimits.maxColorAttachments`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxColorAttachments)
    pub fn max_color_attachments(&self) -> u32 {
        self.inner.get("maxColorAttachments").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxColorAttachmentBytesPerSample` attribute.
    /// [`GPUSupportedLimits.maxColorAttachmentBytesPerSample`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxColorAttachmentBytesPerSample)
    pub fn max_color_attachment_bytes_per_sample(&self) -> u32 {
        self.inner
            .get("maxColorAttachmentBytesPerSample")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxComputeWorkgroupStorageSize` attribute.
    /// [`GPUSupportedLimits.maxComputeWorkgroupStorageSize`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupStorageSize)
    pub fn max_compute_workgroup_storage_size(&self) -> u32 {
        self.inner
            .get("maxComputeWorkgroupStorageSize")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxComputeInvocationsPerWorkgroup` attribute.
    /// [`GPUSupportedLimits.maxComputeInvocationsPerWorkgroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeInvocationsPerWorkgroup)
    pub fn max_compute_invocations_per_workgroup(&self) -> u32 {
        self.inner
            .get("maxComputeInvocationsPerWorkgroup")
            .as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxComputeWorkgroupSizeX` attribute.
    /// [`GPUSupportedLimits.maxComputeWorkgroupSizeX`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupSizeX)
    pub fn max_compute_workgroup_size_x(&self) -> u32 {
        self.inner.get("maxComputeWorkgroupSizeX").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxComputeWorkgroupSizeY` attribute.
    /// [`GPUSupportedLimits.maxComputeWorkgroupSizeY`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupSizeY)
    pub fn max_compute_workgroup_size_y(&self) -> u32 {
        self.inner.get("maxComputeWorkgroupSizeY").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxComputeWorkgroupSizeZ` attribute.
    /// [`GPUSupportedLimits.maxComputeWorkgroupSizeZ`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupSizeZ)
    pub fn max_compute_workgroup_size_z(&self) -> u32 {
        self.inner.get("maxComputeWorkgroupSizeZ").as_::<u32>()
    }
}
impl GPUSupportedLimits {
    /// Getter of the `maxComputeWorkgroupsPerDimension` attribute.
    /// [`GPUSupportedLimits.maxComputeWorkgroupsPerDimension`](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupsPerDimension)
    pub fn max_compute_workgroups_per_dimension(&self) -> u32 {
        self.inner
            .get("maxComputeWorkgroupsPerDimension")
            .as_::<u32>()
    }
}
