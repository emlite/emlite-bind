use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUSupportedLimits {
    inner: emlite::Val,
}
impl FromVal for GPUSupportedLimits {
    fn from_val(v: &emlite::Val) -> Self {
        GPUSupportedLimits { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUSupportedLimits {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUSupportedLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUSupportedLimits {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUSupportedLimits {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<GPUSupportedLimits> for emlite::Val {
    fn from(s: GPUSupportedLimits) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GPUSupportedLimits);


impl GPUSupportedLimits {
    pub fn max_texture_dimension1_d(&self) -> u32 {
        self.inner.get("maxTextureDimension1D").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_texture_dimension2_d(&self) -> u32 {
        self.inner.get("maxTextureDimension2D").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_texture_dimension3_d(&self) -> u32 {
        self.inner.get("maxTextureDimension3D").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_texture_array_layers(&self) -> u32 {
        self.inner.get("maxTextureArrayLayers").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_bind_groups(&self) -> u32 {
        self.inner.get("maxBindGroups").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_bind_groups_plus_vertex_buffers(&self) -> u32 {
        self.inner.get("maxBindGroupsPlusVertexBuffers").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_bindings_per_bind_group(&self) -> u32 {
        self.inner.get("maxBindingsPerBindGroup").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_dynamic_uniform_buffers_per_pipeline_layout(&self) -> u32 {
        self.inner.get("maxDynamicUniformBuffersPerPipelineLayout").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_dynamic_storage_buffers_per_pipeline_layout(&self) -> u32 {
        self.inner.get("maxDynamicStorageBuffersPerPipelineLayout").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_sampled_textures_per_shader_stage(&self) -> u32 {
        self.inner.get("maxSampledTexturesPerShaderStage").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_samplers_per_shader_stage(&self) -> u32 {
        self.inner.get("maxSamplersPerShaderStage").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_storage_buffers_per_shader_stage(&self) -> u32 {
        self.inner.get("maxStorageBuffersPerShaderStage").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_storage_textures_per_shader_stage(&self) -> u32 {
        self.inner.get("maxStorageTexturesPerShaderStage").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_uniform_buffers_per_shader_stage(&self) -> u32 {
        self.inner.get("maxUniformBuffersPerShaderStage").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_uniform_buffer_binding_size(&self) -> u64 {
        self.inner.get("maxUniformBufferBindingSize").as_::<u64>()
    }

}
impl GPUSupportedLimits {
    pub fn max_storage_buffer_binding_size(&self) -> u64 {
        self.inner.get("maxStorageBufferBindingSize").as_::<u64>()
    }

}
impl GPUSupportedLimits {
    pub fn min_uniform_buffer_offset_alignment(&self) -> u32 {
        self.inner.get("minUniformBufferOffsetAlignment").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn min_storage_buffer_offset_alignment(&self) -> u32 {
        self.inner.get("minStorageBufferOffsetAlignment").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_vertex_buffers(&self) -> u32 {
        self.inner.get("maxVertexBuffers").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_buffer_size(&self) -> u64 {
        self.inner.get("maxBufferSize").as_::<u64>()
    }

}
impl GPUSupportedLimits {
    pub fn max_vertex_attributes(&self) -> u32 {
        self.inner.get("maxVertexAttributes").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_vertex_buffer_array_stride(&self) -> u32 {
        self.inner.get("maxVertexBufferArrayStride").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_inter_stage_shader_variables(&self) -> u32 {
        self.inner.get("maxInterStageShaderVariables").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_color_attachments(&self) -> u32 {
        self.inner.get("maxColorAttachments").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_color_attachment_bytes_per_sample(&self) -> u32 {
        self.inner.get("maxColorAttachmentBytesPerSample").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_compute_workgroup_storage_size(&self) -> u32 {
        self.inner.get("maxComputeWorkgroupStorageSize").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_compute_invocations_per_workgroup(&self) -> u32 {
        self.inner.get("maxComputeInvocationsPerWorkgroup").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_compute_workgroup_size_x(&self) -> u32 {
        self.inner.get("maxComputeWorkgroupSizeX").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_compute_workgroup_size_y(&self) -> u32 {
        self.inner.get("maxComputeWorkgroupSizeY").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_compute_workgroup_size_z(&self) -> u32 {
        self.inner.get("maxComputeWorkgroupSizeZ").as_::<u32>()
    }

}
impl GPUSupportedLimits {
    pub fn max_compute_workgroups_per_dimension(&self) -> u32 {
        self.inner.get("maxComputeWorkgroupsPerDimension").as_::<u32>()
    }

}
